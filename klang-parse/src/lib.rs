mod expr;
mod function;
mod lexer;
mod parse;
mod parser;
mod prototype;
mod r#pub;
mod token;

#[cfg(test)]
mod test {
    use klang_ast::{
        expr::Expression,
        function::{Function, Prototype},
        node::ASTNode,
    };

    use crate::{lexer, token::Token};

    use super::parser::parse;

    #[test]
    fn parse_function_definition() {
        let input_str = r#"pub kaya();"#;
        let token_stream = lexer::tokenize(input_str).unwrap();
        let parse_result = parse(&token_stream, &[]).unwrap();
        let expected_tree = vec![ASTNode::ExternNode(Prototype {
            name: "kaya".to_string(),
            args: vec![],
        })];

        let left_tokens = vec![];
        let expected_result = (expected_tree, left_tokens);

        assert_eq!(parse_result, expected_result)
    }

    #[test]
    fn parse_function_definition_with_lefotver_tokens() {
        let input_str = r#"pub kaya(); pub"#;
        let token_stream = lexer::tokenize(input_str).unwrap();
        let parse_result = parse(&token_stream, &[]).unwrap();
        let expected_tree = vec![ASTNode::ExternNode(Prototype {
            name: "kaya".to_string(),
            args: vec![],
        })];

        let left_tokens = vec![Token::Pub];
        let expected_result = (expected_tree, left_tokens);

        assert_eq!(parse_result, expected_result)
    }

    #[test]
    fn parse_function_declaration() {
        let input_str = r#"fun kaya() { 5 }"#;
        let token_stream = lexer::tokenize(input_str).unwrap();
        let parse_result = parse(&token_stream, &[]).unwrap();
        let expected_tree = vec![ASTNode::FunctionNode(Function {
            prototype: Prototype {
                name: "kaya".to_string(),
                args: vec![],
            },
            body: Expression::Literal(5.0),
        })];

        let left_tokens = vec![];
        let expected_result = (expected_tree, left_tokens);

        assert_eq!(parse_result, expected_result)
    }

    #[test]
    fn parse_function_declaration_with_leftover_tokens() {
        let input_str = r#"fun kaya() { 5 } fun"#;
        let token_stream = lexer::tokenize(input_str).unwrap();
        let parse_result = parse(&token_stream, &[]).unwrap();
        let expected_tree = vec![ASTNode::FunctionNode(Function {
            prototype: Prototype {
                name: "kaya".to_string(),
                args: vec![],
            },
            body: Expression::Literal(5.0),
        })];

        let left_tokens = vec![Token::Fun];
        let expected_result = (expected_tree, left_tokens);

        assert_eq!(parse_result, expected_result)
    }
}
