mod run;

use anyhow::Result;
pub use run::Command as RunCommand;
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
    Run(RunCommand),
}

pub fn run_cli() -> Result<()> {
    let opt = Opt::parse();
    match opt.command {
        Korc::Run(run_command) => run::exec(run_command),
    }
}
