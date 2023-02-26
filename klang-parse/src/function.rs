use crate::{
    expect_token,
    parse::{error, Parse, PartParsingResult},
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
        expect_token!(
            [Token::OpeningBrace, Token::OpeningBrace, ()] <= tokens,
            parsed_tokens,
            "'{' expected"
        );

        let expr_partial_parsing = Expression::parse(tokens);
        let body = parse_try!(expr_partial_parsing, tokens, parsed_tokens);
        expect_token!(
            [Token::ClosingBrace, Token::ClosingBrace, ()] <= tokens,
            parsed_tokens,
            "'}' expected"
        );

        PartParsingResult::Good(
            ASTNode::FunctionNode(Function { prototype, body }),
            parsed_tokens,
        )
    }
}
