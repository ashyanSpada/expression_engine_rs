use crate::keyword;
use core::clone::Clone;
use rust_decimal::Decimal;
use std::fmt;

#[derive(Clone, PartialEq, Debug, Copy)]
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

#[derive(Clone, PartialEq, Debug, Copy)]
pub struct Span(pub usize, pub usize);

#[derive(Clone, PartialEq, Debug, Copy)]
pub enum Token<'input> {
    Operator(&'input str, Span),
    Delim(DelimTokenType, Span),
    Number(Decimal, Span),
    Comma(&'input str, Span),
    Bool(bool, Span),
    String(&'input str, Span),
    Reference(&'input str, Span),
    Function(&'input str, Span),
    Semicolon(&'input str, Span),
    EOF,
}

pub fn check_op(token: Token, expected: &str) -> bool {
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

impl<'input> Token<'input> {
    pub fn is_open_paren(self) -> bool {
        check_op(self, "(")
    }

    pub fn is_close_paren(self) -> bool {
        check_op(self, ")")
    }

    pub fn is_open_bracket(self) -> bool {
        check_op(self, "[")
    }

    pub fn is_close_bracket(self) -> bool {
        check_op(self, "]")
    }

    pub fn is_open_brace(self) -> bool {
        check_op(self, "{")
    }

    pub fn is_close_brace(self) -> bool {
        check_op(self, "}")
    }

    pub fn is_question_mark(self) -> bool {
        check_op(self, "?")
    }

    pub fn is_colon(self) -> bool {
        check_op(self, ":")
    }

    pub fn is_eof(self) -> bool {
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

    pub fn is_postfix_op_token(&self) -> bool {
        match self {
            Self::Operator(op, _) => keyword::is_postfix_op(op),
            _ => false,
        }
    }

    pub fn is_binop_token(&self) -> bool {
        match self {
            Self::Operator(op, _) => keyword::is_infix_op(op),
            _ => false,
        }
    }

    pub fn is_not_token(&self) -> bool {
        match self {
            Self::Operator(op, _) => keyword::is_not(op),
            _ => false,
        }
    }

    pub fn is_semicolon(&self) -> bool {
        match self {
            Self::Semicolon(..) => true,
            _ => false,
        }
    }

    #[cfg(not(tarpaulin_include))]
    pub fn string(self) -> String {
        use Token::*;
        match self {
            Operator(op, _) => op.to_string(),
            Number(val, _) => val.to_string(),
            Comma(val, _) => val.to_string(),
            Bool(val, _) => val.to_string(),
            String(val, _) => val.to_string(),
            Reference(val, _) => val.to_string(),
            Function(val, _) => val.to_string(),
            Semicolon(val, _) => val.to_string(),
            Delim(ty, _) => ty.string(),
            EOF => "EOF".to_string(),
        }
    }
}

#[cfg(not(tarpaulin_include))]
impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}..={})", self.0, self.1)
    }
}

#[cfg(not(tarpaulin_include))]
impl<'input> fmt::Display for Token<'input> {
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

#[cfg(test)]
mod tests {
    use super::{DelimTokenType, Span, Token};
    use rstest::rstest;

    #[rstest]
    #[case("(", DelimTokenType::OpenParen)]
    #[case(")", DelimTokenType::CloseParen)]
    #[case("[", DelimTokenType::OpenBracket)]
    #[case("]", DelimTokenType::CloseBracket)]
    #[case("{", DelimTokenType::OpenBrace)]
    #[case("}", DelimTokenType::CloseBrace)]
    #[case("f", DelimTokenType::Unknown)]
    fn test_delim_token_type_from_str(#[case] input: &str, #[case] output: DelimTokenType) {
        assert_eq!(DelimTokenType::from(input), output)
    }

    #[rstest]
    #[case('(', DelimTokenType::OpenParen)]
    #[case(')', DelimTokenType::CloseParen)]
    #[case('[', DelimTokenType::OpenBracket)]
    #[case(']', DelimTokenType::CloseBracket)]
    #[case('{', DelimTokenType::OpenBrace)]
    #[case('}', DelimTokenType::CloseBrace)]
    #[case('b', DelimTokenType::Unknown)]
    fn test_delim_token_type_from_char(#[case] input: char, #[case] output: DelimTokenType) {
        assert_eq!(DelimTokenType::from(input), output)
    }

    #[rstest]
    #[case(Token::Delim(DelimTokenType::OpenParen, Span(0, 0)), true)]
    #[case(Token::Delim(DelimTokenType::CloseParen, Span(0, 0)), false)]
    #[case(Token::Bool(false, Span(0, 0)), false)]
    fn test_is_open_paren(#[case] input: Token, #[case] output: bool) {
        assert_eq!(input.is_open_paren(), output)
    }

    #[rstest]
    #[case(Token::Delim(DelimTokenType::OpenBrace, Span(0, 0)), true)]
    #[case(Token::Delim(DelimTokenType::CloseBrace, Span(0, 0)), false)]
    #[case(Token::Bool(false, Span(0, 0)), false)]
    fn test_is_open_brace(#[case] input: Token, #[case] output: bool) {
        assert_eq!(input.is_open_brace(), output)
    }

    #[rstest]
    #[case(Token::Delim(DelimTokenType::OpenBracket, Span(0, 0)), true)]
    #[case(Token::Delim(DelimTokenType::CloseBracket, Span(0, 0)), false)]
    #[case(Token::Bool(false, Span(0, 0)), false)]
    fn test_is_open_bracket(#[case] input: Token, #[case] output: bool) {
        assert_eq!(input.is_open_bracket(), output)
    }
}
