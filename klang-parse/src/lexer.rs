use crate::token::Token;

type TokenStream = Vec<Token>;

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
        r"(?P<comma>,)|",
        r"(?P<operator>\S)"
    ))?;

    for cap in token_re.captures_iter(&preprocessed) {
        let token = if let Some(ident) = cap.name("ident") {
            match ident.as_str() {
                "fun" => Token::Fun,
                "pub" => Token::Pub,
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
