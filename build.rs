#[allow(dead_code)]
mod opt {
    include!("src/opt.rs");
}
use clap::IntoApp;
use clap_generate::generators::{Bash, Fish, Zsh};

fn main() {
    let mut app = opt::Opt::into_app();

    let name = app.get_name().to_string();
    let outdir = match std::env::var_os("OUT_DIR") {
        None => return,
        Some(outdir) => outdir,
    };

    clap_generate::generate_to::<Bash, _, _>(&mut app, &name, &outdir);
    clap_generate::generate_to::<Zsh, _, _>(&mut app, &name, &outdir);
    clap_generate::generate_to::<Fish, _, _>(&mut app, &name, &outdir);
}
