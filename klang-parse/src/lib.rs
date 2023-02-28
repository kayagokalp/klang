mod expr;
mod function;
pub mod lexer;
mod parse;
pub mod parser;
mod prototype;
mod r#pub;
pub mod token;

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
    fn parse_function_definition_with_leftover_tokens() {
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
            body: Some(Expression::Literal(5.0)),
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
            body: Some(Expression::Literal(5.0)),
        })];

        let left_tokens = vec![Token::Fun];
        let expected_result = (expected_tree, left_tokens);

        assert_eq!(parse_result, expected_result)
    }

    #[test]
    fn parse_expr_literal() {
        let input_str = r#"5"#;
        let token_stream = lexer::tokenize(input_str).unwrap();
        let parse_result = parse(&token_stream, &[]).unwrap();
        let expected_tree = vec![ASTNode::FunctionNode(Function {
            prototype: Prototype {
                name: "".to_string(),
                args: vec![],
            },
            body: Some(Expression::Literal(5.0)),
        })];

        let left_tokens = vec![];
        let expected_result = (expected_tree, left_tokens);

        assert_eq!(parse_result, expected_result)
    }

    #[test]
    fn parse_expr_literal_with_leftover_tokens() {
        let input_str = r#"5 fun"#;
        let token_stream = lexer::tokenize(input_str).unwrap();
        let parse_result = parse(&token_stream, &[]).unwrap();
        let expected_tree = vec![ASTNode::FunctionNode(Function {
            prototype: Prototype {
                name: "".to_string(),
                args: vec![],
            },
            body: Some(Expression::Literal(5.0)),
        })];

        let left_tokens = vec![Token::Fun];
        let expected_result = (expected_tree, left_tokens);

        assert_eq!(parse_result, expected_result)
    }

    #[test]
    fn parse_expr_variable() {
        let input_str = r#"x"#;
        let token_stream = lexer::tokenize(input_str).unwrap();
        let parse_result = parse(&token_stream, &[]).unwrap();
        let expected_tree = vec![ASTNode::FunctionNode(Function {
            prototype: Prototype {
                name: "".to_string(),
                args: vec![],
            },
            body: Some(Expression::Variable("x".to_string())),
        })];

        let left_tokens = vec![];
        let expected_result = (expected_tree, left_tokens);

        assert_eq!(parse_result, expected_result)
    }

    #[test]
    fn parse_expr_variable_with_leftover_tokens() {
        let input_str = r#"x fun"#;
        let token_stream = lexer::tokenize(input_str).unwrap();
        let parse_result = parse(&token_stream, &[]).unwrap();
        let expected_tree = vec![ASTNode::FunctionNode(Function {
            prototype: Prototype {
                name: "".to_string(),
                args: vec![],
            },
            body: Some(Expression::Variable("x".to_string())),
        })];

        let left_tokens = vec![Token::Fun];
        let expected_result = (expected_tree, left_tokens);

        assert_eq!(parse_result, expected_result)
    }

    #[test]
    fn parse_expr_call() {
        let input_str = r#"x()"#;
        let token_stream = lexer::tokenize(input_str).unwrap();
        let parse_result = parse(&token_stream, &[]).unwrap();
        let expected_tree = vec![ASTNode::FunctionNode(Function {
            prototype: Prototype {
                name: "".to_string(),
                args: vec![],
            },
            body: Some(Expression::Call("x".to_string(), vec![])),
        })];

        let left_tokens = vec![];
        let expected_result = (expected_tree, left_tokens);

        assert_eq!(parse_result, expected_result)
    }

    #[test]
    fn parse_expr_call_with_leftover_tokens() {
        let input_str = r#"x() pub"#;
        let token_stream = lexer::tokenize(input_str).unwrap();
        let parse_result = parse(&token_stream, &[]).unwrap();
        let expected_tree = vec![ASTNode::FunctionNode(Function {
            prototype: Prototype {
                name: "".to_string(),
                args: vec![],
            },
            body: Some(Expression::Call("x".to_string(), vec![])),
        })];

        let left_tokens = vec![Token::Pub];
        let expected_result = (expected_tree, left_tokens);

        assert_eq!(parse_result, expected_result)
    }

    #[test]
    fn parse_expr_literal_binary() {
        let input_str = r#"5 + 4 * 2"#;
        let token_stream = lexer::tokenize(input_str).unwrap();
        let parse_result = parse(&token_stream, &[]).unwrap();
        let expected_tree = vec![ASTNode::FunctionNode(Function {
            prototype: Prototype {
                name: "".to_string(),
                args: vec![],
            },
            body: Some(Expression::Binary(
                "+".to_string(),
                Box::new(Expression::Literal(5.0)),
                Box::new(Expression::Binary(
                    "*".to_string(),
                    Box::new(Expression::Literal(4.0)),
                    Box::new(Expression::Literal(2.0)),
                )),
            )),
        })];

        let left_tokens = vec![];
        let expected_result = (expected_tree, left_tokens);

        assert_eq!(parse_result, expected_result)
    }

    #[test]
    fn parse_expr_conditional() {
        let input_str = r#"if 5 { 1 } else {2}"#;
        let token_stream = lexer::tokenize(input_str).unwrap();
        let parse_result = parse(&token_stream, &[]).unwrap();
        let expected_tree = vec![ASTNode::FunctionNode(Function {
            prototype: Prototype {
                name: "".to_string(),
                args: vec![],
            },
            body: Some(Expression::Conditional {
                cond_expr: Box::new(Expression::Literal(5.0)),
                if_block_expr: Box::new(Expression::Literal(1.0)),
                else_block_expr: Box::new(Expression::Literal(2.0)),
            }),
        })];

        let left_tokens = vec![];
        let expected_result = (expected_tree, left_tokens);

        assert_eq!(parse_result, expected_result)
    }

    #[test]
    fn parse_expr_conditional_with_leftover_tokens() {
        let input_str = r#"if 5 {1} else {2} fun"#;
        let token_stream = lexer::tokenize(input_str).unwrap();
        let parse_result = parse(&token_stream, &[]).unwrap();
        let expected_tree = vec![ASTNode::FunctionNode(Function {
            prototype: Prototype {
                name: "".to_string(),
                args: vec![],
            },
            body: Some(Expression::Conditional {
                cond_expr: Box::new(Expression::Literal(5.0)),
                if_block_expr: Box::new(Expression::Literal(1.0)),
                else_block_expr: Box::new(Expression::Literal(2.0)),
            }),
        })];

        let left_tokens = vec![Token::Fun];
        let expected_result = (expected_tree, left_tokens);

        assert_eq!(parse_result, expected_result)
    }
}
