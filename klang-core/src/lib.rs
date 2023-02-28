mod context;
mod ir_builder;
mod module;

use anyhow::{Ok, Result};
use context::Compiler;
use inkwell::{context::Context, memory_buffer::MemoryBuffer, passes::PassManager};
use klang_ast::node::ASTNode;
use klang_parse::{lexer::tokenize, parser::parse, token::Token};

pub type ParseResult = Result<(Vec<ASTNode>, Vec<Token>)>;

#[no_mangle]
pub extern "C" fn printd(x: f64) -> f64 {
    println!("> {x} <");
    x
}

#[no_mangle]
pub extern "C" fn putchard(x: f64) -> f64 {
    print!("{}", x as u8 as char);
    x
}

// Adding the functions above to a global array,
// so Rust compiler won't remove them.
#[used]
static EXTERNAL_FNS: [extern "C" fn(f64) -> f64; 2] = [putchard, printd];

/// Parse the given_input_str and return the complete AST.
pub fn parse_to_ast(input_str: &str) -> ParseResult {
    let token_stream = tokenize(input_str)?;
    let parsed_nodes = vec![];
    parse(&token_stream, &parsed_nodes).map_err(|e| anyhow::anyhow!("{e}"))
}

pub type CodegenResult = Result<MemoryBuffer>;
/// Convert the given AST to llvm-ir.
pub fn ast_to_ir(context: &Context, ast: &[ASTNode]) -> CodegenResult {
    let module = context.create_module("main");
    let builder = context.create_builder();

    let pass_manager = PassManager::create(&module);

    pass_manager.add_instruction_combining_pass();
    pass_manager.add_reassociate_pass();
    pass_manager.add_gvn_pass();
    pass_manager.add_cfg_simplification_pass();
    pass_manager.add_basic_alias_analysis_pass();
    pass_manager.add_promote_memory_to_register_pass();
    pass_manager.add_instruction_combining_pass();
    pass_manager.add_reassociate_pass();

    pass_manager.initialize();

    for element in ast {
        Compiler::compile(context, &builder, &pass_manager, &module, element)
            .map_err(|e| anyhow::anyhow!("{e}"))?;
    }

    let memory_buffered_module = module.write_bitcode_to_memory();
    Ok(memory_buffered_module)
}
