#[derive(PartialEq, Clone, Debug)]
#[allow(dead_code)]
pub enum Expression {
    Literal(f64),
    Variable(String),
    Binary(String, Box<Expression>, Box<Expression>),
    Call(String, Vec<Expression>),
}
