use crate::function::{Function, Prototype};

#[derive(PartialEq, Clone, Debug)]
#[allow(dead_code)]
pub enum ASTNode {
    ExternNode(Prototype),
    FunctionNode(Function),
}
