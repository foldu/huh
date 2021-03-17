use clap::Clap;

#[derive(Clap)]
// FIXME: bin_name needs to be set because clap_generate ignores
// the argument given in generate_to https://github.com/clap-rs/clap/issues/1898
#[clap(name = "huh", bin_name = "huh")]
pub(crate) struct Opt {
    /// Path to flake, will default to finding it starting from the current
    /// working directory walking up
    pub flake: Option<String>,
    #[clap(subcommand)]
    pub subcmd: Subcmd,
}

#[derive(Clap)]
pub(crate) enum Subcmd {
    /// Update flake inputs
    Update {
        /// Don't commit lock file
        #[clap(short, long)]
        no_lock: bool,
        /// List of flake inputs to update. If left empty will update all inputs
        inputs: Vec<String>,
    },
    /// Test out configuration
    Test,
    /// Switch to new configuration
    Switch,
    /// Open a nix repl with contents of current flake imported
    Repl,
    /// Rollback system configuration
    Rollback,
    /// Check flake
    Check,
    /// Garbage collect nix store
    Gc {
        /// Garbage collect everything older than this. Examples are `all` and
        /// `7d` (7days)
        period: String,
    },
    /// Show exports of current flake
    Show,
}
