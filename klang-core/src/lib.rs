mod context;
mod ir_builder;
mod module;

use anyhow::{Ok, Result};
use klang_ast::node::ASTNode;
use klang_parse::{lexer::tokenize, parser::parse, token::Token};

pub type ParseResult = Result<(Vec<ASTNode>, Vec<Token>)>;

/// Parse the given_input_str and return the complete AST.
pub fn parse_to_ast(input_str: &str) -> ParseResult {
    let token_stream = tokenize(input_str)?;
    let parsed_nodes = vec![];
    parse(&token_stream, &parsed_nodes).map_err(|e| anyhow::anyhow!("{e}"))
}

/// Convert the given AST to llvm-ir.
pub fn ast_to_ir() -> Result<()> {
    Ok(())
}
