#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Fun,
    Use,
    Delimiter,
    OpeningParenthesis,
    ClosingParenthesis,
    OpeningBrace,
    ClosingBrace,
    Comma,
    Ident(String),
    Number(f64),
    Operator(String),
    If,
    Else,
}
