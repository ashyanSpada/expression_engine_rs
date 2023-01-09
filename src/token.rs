use rust_decimal::Decimal;
use core::clone::Clone;
use std::fmt;


pub struct Span(pub usize, pub usize);

#[derive(Clone)]
pub enum Token {
    Brace(String, Span),
    Operator(String, Span),
    Literal(Decimal, Span),
    Comma(String, Span),
    Bool(bool, Span),
    String(String, Span),
    Reference(String, Span),
    Function(String, Span),
}

impl Token {
    pub fn is_left_brace(&self) -> bool {
        match self {
            Token::Brace(ch, _) => ch == "(",
            _ => false,
        }
    }

    pub fn is_right_brace(&self) -> bool {
        match self {
            Token::Brace(ch, _) => ch == ")",
            _ => false,
        }
    }

    pub fn is_question_mark(&self) -> bool {
        match self {
            Token::Operator(op, _) => op == "?",
            _ => false,
        }
    }

    pub fn is_colon(&self) -> bool {
        match self {
            Token::Operator(op, _) => op == ":",
            _ => false,
        }
    }

    pub fn string(&self) -> String {
        match self {
            Self::Brace(brace, _) => brace.clone(),
            Self::Operator(op, _) => op.clone(),
            Self::Literal(val, _) => val.to_string(),
            Self::Comma(val, _) => val.to_string(),
            Self::Bool(val, _) => val.to_string(),
            Self::String(val, _) => val.clone(),
            Self::Reference(val, _) => val.clone(),
            Self::Function(val, _) => val.clone(),
        }
    }
}

impl Clone for Span {
    fn clone(&self) -> Self {
        return Self(self.0.clone(), self.1.clone())
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Brace(brace, _) => write!(f, "brace token: {}", brace),
            Self::Bool(val, _) => write!(f, "bool token: {}", val),
            Self::Comma(val, _) => write!(f, "comma token: {}", val),
            Self::Literal(val, _) => write!(f, "literal token: {}", val),
            Self::Operator(val, _) => write!(f, "operator token: {}", val),
            Self::Reference(val, _) => write!(f, "reference token: {}", val),
            Self::Function(val, _) => write!(f, "function token: {}", val),
            Self::String(val, _) => write!(f, "string token: {}", val)
        }
    }
}
