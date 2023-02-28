use crate::token::Token;

#[allow(dead_code)]
type TokenStream = Vec<Token>;

#[allow(dead_code)]
pub fn tokenize(input: &str) -> anyhow::Result<TokenStream> {
    // regex for commentaries (start with #, end with the line end)
    let comment_re = regex::Regex::new(r"(?m)#.*\n")?;
    // remove commentaries from the input stream
    let preprocessed = comment_re.replace_all(input, "\n");

    let mut result = Vec::new();

    // regex for token, just union of straightforward regexes for different token types
    // operators are parsed the same way as identifier and separated later
    let token_re = regex::Regex::new(concat!(
        r"(?P<ident>\p{Alphabetic}\w*)|",
        r"(?P<number>\d+\.?\d*)|",
        r"(?P<delimiter>;)|",
        r"(?P<oppar>\()|",
        r"(?P<clpar>\))|",
        r"(?P<opbrace>\{)|",
        r"(?P<clbrace>\})|",
        r"(?P<comma>,)|",
        r"(?P<operator>\S)"
    ))?;

    for cap in token_re.captures_iter(&preprocessed) {
        let token = if let Some(ident) = cap.name("ident") {
            match ident.as_str() {
                "fun" => Token::Fun,
                "use" => Token::Use,
                "if" => Token::If,
                "else" => Token::Else,
                _ => Token::Ident(ident.as_str().to_string()),
            }
        } else if let Some(number) = cap.name("number") {
            match number.as_str().parse() {
                Ok(number) => Token::Number(number),
                Err(_) => anyhow::bail!("Lexer failed trying to parse number"),
            }
        } else if cap.name("delimiter").is_some() {
            Token::Delimiter
        } else if cap.name("oppar").is_some() {
            Token::OpeningParenthesis
        } else if cap.name("clpar").is_some() {
            Token::ClosingParenthesis
        } else if cap.name("comma").is_some() {
            Token::Comma
        } else if cap.name("opbrace").is_some() {
            Token::OpeningBrace
        } else if cap.name("clbrace").is_some() {
            Token::ClosingBrace
        } else {
            let operator = cap
                .name("operator")
                .ok_or_else(|| anyhow::anyhow!("lexer failed trying to get operator"))?;
            Token::Operator(operator.as_str().to_string())
        };

        result.push(token)
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::tokenize;
    use crate::token::Token;

    #[test]
    fn test_lex_pub_keyword() {
        let input_str = r#"use"#;
        let token_stream = tokenize(input_str).unwrap();
        let expected = vec![Token::Use];
        assert_eq!(token_stream, expected)
    }

    #[test]
    fn test_lex_fun_keyword() {
        let input_str = r#"fun"#;
        let token_stream = tokenize(input_str).unwrap();
        let expected = vec![Token::Fun];
        assert_eq!(token_stream, expected)
    }

    #[test]
    fn test_lex_ident() {
        let input_str = r#"this is a ident"#;
        let token_stream = tokenize(input_str).unwrap();
        let expected = vec![
            Token::Ident("this".to_string()),
            Token::Ident("is".to_string()),
            Token::Ident("a".to_string()),
            Token::Ident("ident".to_string()),
        ];
        assert_eq!(token_stream, expected)
    }

    #[test]
    fn test_lex_paranthesis() {
        let input_str = r#"()"#;
        let token_stream = tokenize(input_str).unwrap();
        let expected = vec![Token::OpeningParenthesis, Token::ClosingParenthesis];
        assert_eq!(token_stream, expected)
    }

    #[test]
    fn test_lex_braces() {
        let input_str = r#"{}"#;
        let token_stream = tokenize(input_str).unwrap();
        let expected = vec![Token::OpeningBrace, Token::ClosingBrace];
        assert_eq!(token_stream, expected)
    }

    #[test]
    fn test_lex_fun_decl() {
        let input_str = r#"fun this_is_a_decl() {}"#;
        let token_stream = tokenize(input_str).unwrap();
        let expected = vec![
            Token::Fun,
            Token::Ident("this_is_a_decl".to_string()),
            Token::OpeningParenthesis,
            Token::ClosingParenthesis,
            Token::OpeningBrace,
            Token::ClosingBrace,
        ];
        assert_eq!(token_stream, expected)
    }

    #[test]
    fn test_lex_number() {
        let input_str = r#"102"#;
        let token_stream = tokenize(input_str).unwrap();
        let expected = vec![Token::Number(102.0)];
        assert_eq!(token_stream, expected)
    }

    #[test]
    fn test_if_else_stmnt() {
        let input_str = r#"if a {} else {}"#;
        let token_stream = tokenize(input_str).unwrap();
        let expected = vec![
            Token::If,
            Token::Ident("a".to_string()),
            Token::OpeningBrace,
            Token::ClosingBrace,
            Token::Else,
            Token::OpeningBrace,
            Token::ClosingBrace,
        ];
        assert_eq!(token_stream, expected)
    }
}
