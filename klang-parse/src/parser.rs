use crate::{
    parse::{Parse, ParsingResult, PartParsingResult},
    token::Token,
};
use klang_ast::{
    function::{Function, Prototype},
    node::ASTNode,
};

pub fn parse(tokens: &[Token], parsed_tree: &[ASTNode]) -> ParsingResult {
    let mut token_stream = tokens.to_vec();
    token_stream.reverse();
    let mut parsed_tree = parsed_tree.to_vec();

    loop {
        // look at the current token and determine what to parse
        // based on its value
        let cur_token = match token_stream.last() {
            Some(token) => token.clone(),
            None => break,
        };

        let result = match cur_token {
            Token::Fun => Function::parse(&mut token_stream),
            Token::Pub => Prototype::parse(&mut token_stream),
            Token::Delimiter => todo!(),
            _ => todo!(),
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
