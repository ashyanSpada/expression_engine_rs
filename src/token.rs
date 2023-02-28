use core::clone::Clone;
use rust_decimal::Decimal;
use std::fmt;

pub struct Span(pub usize, pub usize);

#[derive(Clone)]
pub enum Token {
    Bracket(String, Span),
    Operator(String, Span),
    Number(Decimal, Span),
    Comma(String, Span),
    Bool(bool, Span),
    String(String, Span),
    Reference(String, Span),
    Function(String, Span),
    Semicolon(String, Span),
    EOF,
}

pub fn check_op(token: &Token, expected: &str) -> bool {
    match token {
        Token::Bracket(op, _) => {
            if op == expected {
                return true;
            }
        }
        Token::Operator(op, _) => {
            if op == expected {
                return true;
            }
        }
        _ => return false,
    }
    return false;
}

impl Token {
    pub fn is_left_paren(&self) -> bool {
        check_op(self, "(")
    }

    pub fn is_right_paren(&self) -> bool {
        check_op(self, ")")
    }

    pub fn is_left_bracket(&self) -> bool {
        check_op(self, "[")
    }

    pub fn is_right_bracket(&self) -> bool {
        check_op(self, "]")
    }

    pub fn is_left_curly(&self) -> bool {
        check_op(self, "{")
    }

    pub fn is_right_curly(&self) -> bool {
        check_op(self, "}")
    }

    pub fn is_question_mark(&self) -> bool {
        check_op(self, "?")
    }

    pub fn is_colon(&self) -> bool {
        check_op(self, ":")
    }

    pub fn is_eof(&self) -> bool {
        match self {
            Self::EOF => true,
            _ => false,
        }
    }

    pub fn is_op_token(&self) -> bool {
        match self {
            Self::Operator(_, _) => true,
            _ => false,
        }
    }

    pub fn string(&self) -> String {
        match self {
            Self::Bracket(bracket, _) => bracket.clone(),
            Self::Operator(op, _) => op.clone(),
            Self::Number(val, _) => val.to_string(),
            Self::Comma(val, _) => val.to_string(),
            Self::Bool(val, _) => val.to_string(),
            Self::String(val, _) => val.clone(),
            Self::Reference(val, _) => val.clone(),
            Self::Function(val, _) => val.clone(),
            Self::Semicolon(val, _) => val.clone(),
            Self::EOF => "EOF".to_string(),
        }
    }
}

impl Clone for Span {
    fn clone(&self) -> Self {
        return Self(self.0.clone(), self.1.clone());
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bracket(bracket, _) => write!(f, "Bracket Token: {}", bracket),
            Self::Bool(val, _) => write!(f, "Bool Token: {}", val),
            Self::Comma(val, _) => write!(f, "Comma Token: {}", val),
            Self::Number(val, _) => write!(f, "Number Token: {}", val),
            Self::Operator(val, _) => write!(f, "Operator Token: {}", val),
            Self::Reference(val, _) => write!(f, "Reference Token: {}", val),
            Self::Function(val, _) => write!(f, "Function Token: {}", val),
            Self::String(val, _) => write!(f, "String Token: {}", val),
            Self::Semicolon(val, _) => write!(f, "Semicolon Token: {}", val),
            Self::EOF => write!(f, "EOF"),
        }
    }
}
