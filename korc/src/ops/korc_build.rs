use anyhow::Result;
use crate::cli::BuildCommand;

pub fn build(cmd: BuildCommand) -> Result<()> {
    println!("building");
    Ok(())
}
