use core::clone::Clone;
use rust_decimal::Decimal;
use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub enum DelimTokenType {
    // "("
    OpenParen,
    // ")"
    CloseParen,
    // "["
    OpenBracket,
    // "]"
    CloseBracket,
    // "{"
    OpenBrace,
    // "}"
    CloseBrace,

    Unknown,
}

impl From<char> for DelimTokenType {
    fn from(value: char) -> Self {
        use DelimTokenType::*;
        match value {
            '(' => OpenParen,
            ')' => CloseParen,
            '[' => OpenBracket,
            ']' => CloseBracket,
            '{' => OpenBrace,
            '}' => CloseBrace,
            _ => Unknown,
        }
    }
}

impl From<&str> for DelimTokenType {
    fn from(value: &str) -> Self {
        use DelimTokenType::*;
        match value {
            "(" => OpenParen,
            ")" => CloseParen,
            "[" => OpenBracket,
            "]" => CloseBracket,
            "{" => OpenBrace,
            "}" => CloseBrace,
            _ => Unknown,
        }
    }
}

impl DelimTokenType {
    pub fn string(&self) -> String {
        use DelimTokenType::*;
        match self {
            OpenParen => "(".to_string(),
            CloseParen => ")".to_string(),
            OpenBracket => "[".to_string(),
            CloseBracket => "]".to_string(),
            OpenBrace => "{".to_string(),
            CloseBrace => "}".to_string(),
            Unknown => "??".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Span(pub usize, pub usize);

#[derive(Clone, PartialEq, Debug)]
pub enum Token {
    Operator(String, Span),
    Delim(DelimTokenType, Span),
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
        Token::Delim(op, _) => {
            if op.string() == expected {
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
    pub fn is_open_paren(&self) -> bool {
        check_op(self, "(")
    }

    pub fn is_close_paren(&self) -> bool {
        check_op(self, ")")
    }

    pub fn is_open_bracket(&self) -> bool {
        check_op(self, "[")
    }

    pub fn is_close_bracket(&self) -> bool {
        check_op(self, "]")
    }

    pub fn is_open_brace(&self) -> bool {
        check_op(self, "{")
    }

    pub fn is_close_brace(&self) -> bool {
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
            Self::Operator(op, _) => true,
            _ => false,
        }
    }

    pub fn is_binop_token(&self) -> bool {
        match self {
            Self::Operator(op, _) => op != "?" && op != ":",
            _ => false,
        }
    }

    pub fn is_semicolon(&self) -> bool {
        match self {
            Self::Semicolon(..) => true,
            _ => false,
        }
    }

    pub fn string(&self) -> String {
        use Token::*;
        match self {
            Operator(op, _) => op.clone(),
            Number(val, _) => val.to_string(),
            Comma(val, _) => val.to_string(),
            Bool(val, _) => val.to_string(),
            String(val, _) => val.clone(),
            Reference(val, _) => val.clone(),
            Function(val, _) => val.clone(),
            Semicolon(val, _) => val.clone(),
            Delim(ty, _) => ty.string(),
            EOF => "EOF".to_string(),
        }
    }
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}..={})", self.0, self.1)
    }
}

#[cfg(not(tarpaulin_include))]
impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Token::*;
        match self {
            Bool(val, span) => write!(f, "Bool Token: {}, {}", val, span),
            Comma(val, span) => write!(f, "Comma Token: {}, {}", val, span),
            Number(val, span) => write!(f, "Number Token: {}, {}", val, span),
            Operator(val, span) => write!(f, "Operator Token: {}, {}", val, span),
            Reference(val, span) => write!(f, "Reference Token: {}, {}", val, span),
            Function(val, span) => write!(f, "Function Token: {}, {}", val, span),
            String(val, span) => write!(f, "String Token: {}, {}", val, span),
            Semicolon(val, span) => write!(f, "Semicolon Token: {}, {}", val, span),
            Delim(ty, span) => write!(f, "Delim Token: {}, {}", ty.string(), span),
            EOF => write!(f, "EOF"),
        }
    }
}
