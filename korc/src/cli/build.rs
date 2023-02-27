use anyhow::Result;
use clap::Parser;

/// Compile the current or target project.
#[derive(Debug, Default, Parser)]
pub struct Command {
    #[clap(long)]
    pub repl: bool,
}

pub(crate) fn exec(build_command: Command) -> Result<()> {
    crate::ops::korc_build::build(build_command)
}
