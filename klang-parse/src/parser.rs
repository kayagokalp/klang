use std::collections::HashMap;

use crate::{
    parse::{Parse, ParsingResult, PartParsingResult},
    token::Token,
};
use klang_ast::{
    expr::Expression,
    function::{Function, Prototype},
    node::ASTNode,
};

#[derive(Debug)]
pub struct ParserSettings {
    pub operator_precedence: HashMap<String, i32>,
}

impl Default for ParserSettings {
    fn default() -> Self {
        let mut operator_precedence = HashMap::new();
        operator_precedence.insert("<".to_string(), 10);
        operator_precedence.insert("+".to_string(), 20);
        operator_precedence.insert("-".to_string(), 20);
        operator_precedence.insert("*".to_string(), 40);

        Self {
            operator_precedence,
        }
    }
}

#[allow(dead_code)]
pub fn parse(tokens: &[Token], parsed_tree: &[ASTNode]) -> ParsingResult {
    let mut token_stream = tokens.to_vec();
    token_stream.reverse();
    let mut parsed_tree = parsed_tree.to_vec();

    while let Some(token) = token_stream.last() {
        let result = match token {
            Token::Fun => Function::parse(&mut token_stream),
            Token::Use => Prototype::parse(&mut token_stream),
            Token::Delimiter => {
                token_stream.pop();
                continue;
            }
            _ => Expression::parse(&mut token_stream),
        };

        match result {
            PartParsingResult::Good(ast_node, _) => parsed_tree.push(ast_node),
            PartParsingResult::NotComplete => break,
            PartParsingResult::Bad(message) => return Err(message),
        }
    }

    token_stream.reverse();
    Ok((parsed_tree, token_stream))
}
