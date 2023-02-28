#[derive(PartialEq, Clone, Debug)]
#[allow(dead_code)]
pub enum Expression {
    Literal(f64),
    Variable(String),
    Binary(String, Box<Expression>, Box<Expression>),
    Call(String, Vec<Expression>),
    Conditional {
        cond_expr: Box<Expression>,
        if_block_expr: Box<Expression>,
        else_block_expr: Box<Expression>,
    },
}
