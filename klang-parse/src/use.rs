use crate::{
    parse::{Parse, PartParsingResult},
    parse_try,
    token::Token,
};
use klang_ast::{function::Prototype, node::ASTNode};

impl Parse<ASTNode> for Prototype {
    fn parse(tokens: &mut Vec<Token>) -> PartParsingResult<ASTNode> {
        tokens.pop();
        let mut parsed_tokens = vec![Token::Use];
        let prototype_partial_parsing = Prototype::parse(tokens);
        let prototype = parse_try!(prototype_partial_parsing, tokens, parsed_tokens);
        PartParsingResult::Good(ASTNode::ExternNode(prototype), parsed_tokens)
    }
}
