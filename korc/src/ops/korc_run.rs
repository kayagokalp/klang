use crate::cli::RunCommand;
use anyhow::Result;
use inkwell::{context::Context, OptimizationLevel};
use klang_core::{ast_to_ir, parse_to_ast};

const KLANG_EXTENSION: &str = ".kl";
const KLANG_ENTRY_NAME: &str = "main";
const KLANG_DEFAULT_AST_FILE_NAME: &str = ".ast";
const KLANG_DEFAULT_BC_EXTENSION: &str = ".bc";

pub fn run(cmd: RunCommand) -> Result<()> {
    if cmd.repl {
        anyhow::bail!("REPL is not supported yet!")
    }

    let current_dir = std::env::current_dir()?;
    let main_file_name = format!("{KLANG_ENTRY_NAME}{KLANG_EXTENSION}");
    let main_file_path = current_dir.join(main_file_name);
    let input_src = std::fs::read_to_string(main_file_path)?;
    let parse_result = parse_to_ast(&input_src)?;
    if cmd.ast {
        let ast_str = format!("AST {parse_result:#?}");
        if cmd.file_out {
            let path = current_dir.join(KLANG_DEFAULT_AST_FILE_NAME);
            std::fs::write(path, ast_str)?;
        } else {
            println!("{ast_str}");
        }
    }
    let (ast, _) = parse_result;
    let context = Context::create();
    let ir_output_module_mb = ast_to_ir(&context, &ast)?;
    let module = context
        .create_module_from_ir(ir_output_module_mb)
        .map_err(|e| anyhow::anyhow!("{e:?}"))?;
    if cmd.ir {
        if cmd.file_out {
            let file_name = format!("{KLANG_ENTRY_NAME}{KLANG_DEFAULT_BC_EXTENSION}");
            let path = current_dir.join(file_name);
            let success = module.write_bitcode_to_path(&path);
            if !success {
                anyhow::bail!("failed to write bitcode into file")
            }
        } else {
            module.print_to_stderr()
        }
    } else {
        let name = "main";

        let execution_engine = module
            .create_jit_execution_engine(OptimizationLevel::None)
            .map_err(|e| anyhow::anyhow!("{e:?}"))?;
        let compiled_fn =
            unsafe { execution_engine.get_function::<unsafe extern "C" fn() -> f64>(name) }?;
        unsafe {
            compiled_fn.call();
        }
    }
    Ok(())
}
