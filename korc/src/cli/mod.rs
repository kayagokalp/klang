mod build;

use anyhow::Result;
pub use build::Command as BuildCommand;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(name = "korc", about = "Klang Orchestrator", version)]
struct Opt {
    /// The command to run
    #[clap(subcommand)]
    command: Korc,
}

#[derive(Subcommand, Debug)]
enum Korc {
    Build(BuildCommand),
}

pub fn run_cli() -> Result<()> {
    let opt = Opt::parse();
    match opt.command {
        Korc::Build(build_command) => build::exec(build_command),
    }
}
