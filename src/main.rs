mod opt;

use clap::Clap;
use eyre::Context;
use std::{io::Write, os::unix::process::CommandExt, process::Command};

fn main() -> Result<(), eyre::Error> {
    let opt = opt::Opt::parse();

    let flake_root = &opt
        .flake
        .map_or_else(find_flake_root, |path| Ok(path.to_owned()))?;

    use opt::Subcmd::*;
    match opt.subcmd {
        Update { no_lock, inputs } => {
            let mut args = Vec::with_capacity(1 + inputs.len() * 2 + usize::from(!no_lock));

            if !no_lock {
                args.push("--commit-lockfile");
            }

            if inputs.is_empty() {
                args.push("update");
            } else {
                args.push("lock");
                for input in &inputs {
                    args.push("--update-input");
                    args.push(input);
                }
            };

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

        Repl => {
            let mut tmp =
                tempfile::NamedTempFile::new().context("Couldn't create temporary file")?;
            let msg = "Couldn't write to tempfile";
            write!(tmp, r#"(builtins.getFlake "{}")"#, flake_root).context(msg)?;
            tmp.flush().context(msg)?;

            let path = tmp.path().to_str().expect("Temporary file not utf-8");
            exec(
                Command::new("nix")
                    .args(["repl", "<nixpkgs>"].iter())
                    .arg(path),
            )
        }
    }
}

fn exec(cmd: &mut Command) -> Result<(), eyre::Error> {
    // FIXME: put command name into context when `command_access` is stable
    Err(cmd.exec()).context("Could not find command")
}

fn rebuild(kind: &str, flake_root: &str, extra_args: &[&str]) -> Result<(), eyre::Error> {
    let code = Command::new("doas")
        .args(["nixos-rebuild", kind, "--flake", flake_root].iter())
        .args(extra_args)
        .status()
        .context("Could not find doas")?;
    if kind == "switch" && code.success() {
        std::fs::remove_file("result").context("Could not remove result link")?;
    }
    std::process::exit(code.code().unwrap_or(1));
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
