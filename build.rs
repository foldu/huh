use clap::IntoApp;
use clap_complete::shells::*;

include!("src/opt.rs");

const BIN: &str = env!("CARGO_PKG_NAME");

fn main() {
    let mut opt = Opt::into_app();

    let outdir = match std::env::var_os("OUT_DIR") {
        None => return,
        Some(outdir) => outdir,
    };

    clap_complete::generate_to(Bash, &mut opt, BIN, &outdir).unwrap();
    clap_complete::generate_to(Fish, &mut opt, BIN, &outdir).unwrap();
    clap_complete::generate_to(Zsh, &mut opt, BIN, &outdir).unwrap();
}
