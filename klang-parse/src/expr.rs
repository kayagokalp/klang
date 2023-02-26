use crate::{
    parse::{Parse, PartParsingResult, error},
    parse_try,
    token::Token, expect_token,
};
use klang_ast::{
    expr::Expression,
    function::{Function, Prototype},
    node::ASTNode,
};

impl Parse<ASTNode> for Expression {
    fn parse(tokens: &mut Vec<Token>) -> PartParsingResult<ASTNode> {
        let mut parsed_tokens = Vec::new();
        let expression_partial = Expression::parse(tokens);
        let expression = parse_try!(expression_partial, tokens, parsed_tokens);
        let prototype = Prototype {
            name: "".to_string(),
            args: vec![],
        };
        let lambda = Function {
            prototype,
            body: expression,
        };

        PartParsingResult::Good(ASTNode::FunctionNode(lambda), parsed_tokens)
    }
}

impl Parse<Expression> for Expression {
    fn parse(tokens: &mut Vec<Token>) -> PartParsingResult<Expression> {
        match tokens.last() {
            Some(&Token::Ident(_)) => parse_ident_expr(tokens),
            Some(&Token::Number(_)) => parse_literal_expr(tokens),
            Some(&Token::OpeningParenthesis) => parse_parenthesis_expr(tokens),
            None => PartParsingResult::NotComplete,
            _ => error("unknow token when expecting an expression")
        }
    }
}

fn parse_ident_expr(tokens: &mut Vec<Token>) -> PartParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();
    let name = expect_token!(
        [Token::Ident(name), Token::Ident(name.clone()), name] <= tokens,
        parsed_tokens, "identificator expected");

     expect_token!(
        [Token::OpeningParenthesis, Token::OpeningParenthesis, ()]
        else {return PartParsingResult::Good(Expression::Variable(name), parsed_tokens)}
        <= tokens, parsed_tokens);

    let mut args = Vec::new();
    loop {
        expect_token!(
            [Token::ClosingParenthesis, Token::ClosingParenthesis, break;
             Token::Comma, Token::Comma, continue]
            else {
                let expr_partial_parsing: PartParsingResult<Expression> = Expression::parse(tokens);
                let expr = parse_try!(expr_partial_parsing, tokens, parsed_tokens);
                args.push(expr);
            }
            <= tokens, parsed_tokens);
    }
    PartParsingResult::Good(Expression::Call(name, args), parsed_tokens)
}

fn parse_literal_expr(tokens: &mut Vec<Token>) -> PartParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();

    let value = expect_token!(
        [Token::Number(val), Token::Number(val), val] <= tokens,
        parsed_tokens, "literal expected");

    PartParsingResult::Good(Expression::Literal(value), parsed_tokens)
}

fn parse_parenthesis_expr(tokens : &mut Vec<Token>) -> PartParsingResult<Expression> {
    // Consume `(`.
    tokens.pop();
    let mut parsed_tokens = vec![Token::OpeningParenthesis];
    let expr: PartParsingResult<Expression> = Expression::parse(tokens);
    let expr = parse_try!(expr, tokens, parsed_tokens);

    expect_token!(
        [Token::ClosingParenthesis, Token::ClosingParenthesis, ()] <= tokens,
        parsed_tokens, "')' expected");

    PartParsingResult::Good(expr, parsed_tokens)
}
