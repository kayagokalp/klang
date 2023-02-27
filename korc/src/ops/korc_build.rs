use crate::cli::BuildCommand;
use anyhow::Result;

use klang_core::parse_to_ast;

const KLANG_EXTENSION: &str = ".kl";
const KLANG_ENTRY_NAME: &str = "main";

pub fn build(cmd: BuildCommand) -> Result<()> {
    if cmd.repl {
        anyhow::bail!("REPL is not supported yet!")
    }

    let current_dir = std::env::current_dir()?;
    let main_file_name = format!("{KLANG_ENTRY_NAME}{KLANG_EXTENSION}");
    let main_file_path = current_dir.join(main_file_name);
    let input_src = std::fs::read_to_string(main_file_path)?;
    let parse_result = parse_to_ast(&input_src)?;
    if cmd.ast {
        println!("AST {parse_result:#?}");
    }
    Ok(())
}
