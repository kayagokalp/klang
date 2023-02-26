use crate::{
    expect_token,
    parse::{error, Parse, PartParsingResult},
    token::Token,
};
use klang_ast::function::Prototype;

impl Parse<Prototype> for Prototype {
    fn parse(tokens: &mut Vec<Token>) -> PartParsingResult<Prototype> {
        let mut parsed_tokens = Vec::new();

        let name = expect_token!(
            [Token::Ident(name), Token::Ident(name.clone()), name] <= tokens,
            parsed_tokens,
            "expected function name in prototype"
        );

        expect_token!(
            [Token::OpeningParenthesis, Token::OpeningParenthesis, ()] <= tokens,
            parsed_tokens,
            "expected '(' in prototype"
        );

        let mut args = Vec::new();
        loop {
            expect_token!([
            Token::Ident(arg), Token::Ident(arg.clone()), args.push(arg.clone());
            Token::Comma, Token::Comma, continue;
            Token::ClosingParenthesis, Token::ClosingParenthesis, break
        ] <= tokens, parsed_tokens, "expected ')' in prototype");
        }

        PartParsingResult::Good(Prototype { name, args }, parsed_tokens)
    }
}
