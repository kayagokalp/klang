#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    Fun,
    Pub,
    Delimiter, //';' character
    OpeningParenthesis,
    ClosingParenthesis,
    Comma,
    Ident(String),
    Number(f64),
    Operator(String),
}
