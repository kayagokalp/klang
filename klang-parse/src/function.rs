use crate::{
    parse::{Parse, PartParsingResult},
    parse_try,
    token::Token,
};
use klang_ast::{
    expr::Expression,
    function::{Function, Prototype},
    node::ASTNode,
};

impl Parse<ASTNode> for Function {
    fn parse(tokens: &mut Vec<Token>) -> PartParsingResult<ASTNode> {
        // Consume `fun` keyword.
        tokens.pop();
        let mut parsed_tokens = vec![Token::Fun];
        let prototype_partial_parsing = Prototype::parse(tokens);
        let prototype = parse_try!(prototype_partial_parsing, tokens, parsed_tokens);

        let expr_partial_parsing = Expression::parse(tokens);
        let body = parse_try!(expr_partial_parsing, tokens, parsed_tokens);

        PartParsingResult::Good(
            ASTNode::FunctionNode(Function { prototype, body }),
            parsed_tokens,
        )
    }
}
