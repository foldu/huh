use clap::Clap;

#[derive(Clap)]
#[clap(name = "hey", bin_name = "hey")]
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
}
