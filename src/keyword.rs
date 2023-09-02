use crate::operator::{BinaryOpFuncManager, PostfixOpFuncManager, UnaryOpFuncManager};

pub fn is_prefix_op(op: &str) -> bool {
    UnaryOpFuncManager::new().exist(op)
}

pub fn is_infix_op(op: &str) -> bool {
    BinaryOpFuncManager::new().exist(op)
}

pub fn is_postfix_op(op: &str) -> bool {
    PostfixOpFuncManager::new().exist(op)
}

pub fn is_ternary_op(op: &str) -> bool {
    op == "?" || op == ":"
}

pub fn is_op(op: &str) -> bool {
    is_prefix_op(op) || is_infix_op(op) || is_postfix_op(op) || is_ternary_op(op)
}

pub fn is_not(op: &str) -> bool {
    op == "not"
}
