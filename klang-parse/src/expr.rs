use crate::{
    expect_token,
    parse::{error, Parse, PartParsingResult},
    parse_try,
    parser::ParserSettings,
    token::Token,
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
        let mut parsed_tokens = Vec::new();
        let lhs_partial_parse = parse_primary_expr(tokens);
        let lhs = parse_try!(lhs_partial_parse, tokens, parsed_tokens);

        let starting_precedence = 0;
        let expr_partial_parse = parse_binary_expr(tokens, starting_precedence, &lhs);
        let expr = parse_try!(expr_partial_parse, tokens, parsed_tokens);
        PartParsingResult::Good(expr, parsed_tokens)
    }
}

fn parse_binary_expr(
    tokens: &mut Vec<Token>,
    expr_precedence: i32,
    lhs: &Expression,
) -> PartParsingResult<Expression> {
    let mut result = lhs.clone();
    let parser_settings = ParserSettings::default();
    let mut parsed_tokens = Vec::new();
    loop {
        // continue until the current token is not an operator
        // or it is an operator with precedence lesser than expr_precedence
        let (operator, precedence) = match tokens.last() {
            Some(&Token::Operator(ref op)) => match parser_settings.operator_precedence.get(op) {
                Some(pr) if *pr >= expr_precedence => (op.clone(), *pr),
                None => return error("unknown operator found"),
                _ => break,
            },
            _ => break,
        };
        tokens.pop();
        parsed_tokens.push(Token::Operator(operator.clone()));

        // parse primary RHS expression
        let rhs_partial_parse = parse_primary_expr(tokens);
        let mut rhs = parse_try!(rhs_partial_parse, tokens, parsed_tokens);
        // parse all the RHS operators until their precedence is
        // bigger than the current one
        loop {
            let binary_rhs = match tokens.last().map(|i| i.clone()) {
                Some(Token::Operator(ref op)) => {
                    match parser_settings.operator_precedence.get(op).map(|i| *i) {
                        Some(pr) if pr > precedence => {
                            let binary_expr_partial_parse =
                                parse_binary_expr(tokens, expr_precedence, &rhs);
                            parse_try!(binary_expr_partial_parse, tokens, parsed_tokens)
                        }
                        None => return error("unknown operator found"),
                        _ => break,
                    }
                }
                _ => break,
            };

            rhs = binary_rhs;
        }
        result = Expression::Binary(operator, Box::new(result), Box::new(rhs));
    }

    PartParsingResult::Good(result, parsed_tokens)
}

fn parse_primary_expr(tokens: &mut Vec<Token>) -> PartParsingResult<Expression> {
    match tokens.last() {
        Some(&Token::Ident(_)) => parse_ident_expr(tokens),
        Some(&Token::Number(_)) => parse_literal_expr(tokens),
        Some(&Token::OpeningParenthesis) => parse_parenthesis_expr(tokens),
        None => PartParsingResult::NotComplete,
        _ => error("unknown token when expecting an expression"),
    }
}

fn parse_ident_expr(tokens: &mut Vec<Token>) -> PartParsingResult<Expression> {
    let mut parsed_tokens = Vec::new();
    let name = expect_token!(
        [Token::Ident(name), Token::Ident(name.clone()), name] <= tokens,
        parsed_tokens,
        "identificator expected"
    );

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
        parsed_tokens,
        "literal expected"
    );

    PartParsingResult::Good(Expression::Literal(value), parsed_tokens)
}

fn parse_parenthesis_expr(tokens: &mut Vec<Token>) -> PartParsingResult<Expression> {
    // Consume `(`.
    tokens.pop();
    let mut parsed_tokens = vec![Token::OpeningParenthesis];
    let expr: PartParsingResult<Expression> = Expression::parse(tokens);
    let expr = parse_try!(expr, tokens, parsed_tokens);

    expect_token!(
        [Token::ClosingParenthesis, Token::ClosingParenthesis, ()] <= tokens,
        parsed_tokens,
        "')' expected"
    );

    PartParsingResult::Good(expr, parsed_tokens)
}
