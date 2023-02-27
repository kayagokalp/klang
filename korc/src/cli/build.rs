use anyhow::Result;
use clap::Parser;

/// Compile the current or target project.
#[derive(Debug, Default, Parser)]
pub struct Command {
    /// Start REPL mode.
    #[clap(long)]
    pub repl: bool,
    /// Print generated AST.
    #[clap(long)]
    pub ast: bool,
}

pub(crate) fn exec(build_command: Command) -> Result<()> {
    crate::ops::korc_build::build(build_command)
}
