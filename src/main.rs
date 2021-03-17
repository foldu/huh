mod opt;

use clap::Clap;
use eyre::Context;
use std::{io::Write, os::unix::process::CommandExt, path::Path, process::Command};

fn main() -> Result<(), eyre::Error> {
    let opt = opt::Opt::parse();

    let flake_root = &opt
        .flake
        .map_or_else(find_flake_root, |path| Ok(path.to_owned()))?;

    use opt::Subcmd::*;
    match opt.subcmd {
        Update { no_lock, inputs } => {
            let mut args = Vec::with_capacity(1 + inputs.len() * 2 + usize::from(!no_lock));

            if inputs.is_empty() {
                args.push("update");
            } else {
                args.push("lock");
                for input in &inputs {
                    args.push("--update-input");
                    args.push(input);
                }
            };

            if !no_lock {
                args.push("--commit-lock-file");
            }

            exec(
                Command::new("nix")
                    .arg("flake")
                    .current_dir(&flake_root)
                    .args(&args),
            )
        }

        Test => rebuild("test", &flake_root, &["--fast"]),

        Switch => rebuild("switch", &flake_root, &[]),

        Rollback => rebuild("switch", &flake_root, &["--rollback"]),

        Gc { period } => {
            let mut cmd = privileged()?;
            cmd.arg("nix-collect-garbage");
            let cmd = if period == "all" {
                cmd.arg("-d")
            } else {
                cmd.args(&["--delete-older-than", period.as_str()])
            };
            exec(cmd)
        }

        Repl => {
            let mut tmp =
                tempfile::NamedTempFile::new().context("Couldn't create temporary file")?;
            let msg = "Couldn't write to tempfile";
            write!(tmp, r#"(builtins.getFlake "{}")"#, flake_root).context(msg)?;
            tmp.flush().context(msg)?;

            let path = tmp.path().to_str().expect("Temporary file not utf-8");
            exec(Command::new("nix").args(&["repl", "<nixpkgs>"]).arg(path))
        }

        Check => exec(Command::new("nix").args(&["flake", "check"])),

        Show => exec(Command::new("nix").args(&["flake", "show"])),
    }
}

fn exec(cmd: &mut Command) -> Result<(), eyre::Error> {
    // FIXME: put command name into context when `command_access` is stable
    Err(cmd.exec()).context("Could not find command")
}

fn rebuild(kind: &str, flake_root: &str, extra_args: &[&str]) -> Result<(), eyre::Error> {
    let code = privileged()?
        .args(&["nixos-rebuild", kind, "--flake", flake_root])
        .args(extra_args)
        .status()
        .expect("Privilege escalation utility vanished");
    if code.success() {
        // FIXME: sometimes it creates a result file sometimes it doesn't
        // I don't get it
        // only remove if symlink as to not accidentally nuke something important
        remove_if_exists_and_symlink("result")?;
    }
    std::process::exit(code.code().unwrap_or(1));
}

// FIXME: bad function name
fn remove_if_exists_and_symlink(path: impl AsRef<Path>) -> Result<(), eyre::Error> {
    let path = path.as_ref();
    let meta = match std::fs::metadata(path) {
        Ok(meta) => meta,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
        Err(e) => return Err(e).with_context(|| format!("Could not stat {}", path.display())),
    };

    if meta.file_type().is_symlink() {
        match std::fs::remove_file(path) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(e).with_context(|| format!("Could not remove {}", path.display())),
        }
    } else {
        Ok(())
    }
}

fn privileged() -> Result<Command, eyre::Error> {
    match ["doas", "sudo"]
        .iter()
        .find_map(|cmd| which::which(cmd).ok())
    {
        Some(path) => Ok(Command::new(path)),
        None => {
            let su = which::which("su")
                .context("Could not find any privilege escalation utilities (doas|sudo|su)")?;
            // TODO: check out if this even works
            let mut cmd = Command::new(su);
            cmd.arg("-c");
            Ok(cmd)
        }
    }
}

fn find_flake_root() -> Result<String, eyre::Error> {
    let cwd = std::env::current_dir().context("Can't get current working directory")?;

    let mut base = cwd.as_path();
    loop {
        let path = base.join("flake.nix");
        if path.exists() {
            break base
                .to_str()
                .ok_or_else(|| eyre::format_err!("Flake base path is invalid utf-8"))
                .map(ToOwned::to_owned);
        }
        match base.parent() {
            Some(parent) => {
                base = parent;
            }
            None => {
                eyre::bail!("Could not find flake root");
            }
        }
    }
}
