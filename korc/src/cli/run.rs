use anyhow::Result;
use clap::Parser;

/// Compile and run the current project. 
#[derive(Debug, Default, Parser)]
pub struct Command {
    /// Start REPL mode.
    #[clap(long)]
    pub repl: bool,
    /// Output generated AST.
    #[clap(long)]
    pub ast: bool,
    /// Output generated IR.
    #[clap(long)]
    pub ir: bool,
    /// Output to file.
    #[clap(long)]
    pub file_out: bool,
}

pub(crate) fn exec(run_command: Command) -> Result<()> {
    crate::ops::korc_run::run(run_command)
}
