use crate::define::Result;
use crate::error::Error;
use crate::keyword;
use crate::token::{Span, Token};
use rust_decimal::prelude::*;
use std::str;

#[derive(Clone)]
pub struct Tokenizer<'a> {
    input: &'a str,
    chars: str::CharIndices<'a>,
    cur_char: char,
    pub cur_token: Token<'a>,
    pub prev_token: Token<'a>,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &str) -> Tokenizer {
        Tokenizer {
            input: input,
            chars: input.char_indices(),
            cur_char: ' ',
            cur_token: Token::EOF,
            prev_token: Token::EOF,
        }
    }

    fn next_one(&mut self) -> Option<(usize, char)> {
        let (cur, cur_char) = self.chars.next()?;
        self.cur_char = cur_char;
        Some((cur, cur_char))
    }

    fn peek_one(&mut self) -> Option<(usize, char)> {
        self.chars.clone().next()
    }

    pub fn next(&mut self) -> Result<Token<'a>> {
        self.eat_whitespace();
        self.prev_token = self.cur_token;
        self.cur_token = match self.next_one() {
            Some((
                start,
                '+' | '-' | '*' | '/' | '^' | '%' | '&' | '!' | '=' | '?' | ':' | '>' | '<' | '|',
            )) => self.special_op_token(start),
            Some((start, '(' | ')' | '[' | ']' | '{' | '}')) => self.delim_token(start),
            Some((start, _ch @ '0'..='9')) => self.number_token(start),
            Some((start, '"' | '\'')) => self.string_token(start),
            Some((start, ';')) => self.semicolon_token(start),
            Some((start, ',')) => self.comma_token(start),
            None => Ok(Token::EOF),
            Some((start, ch)) => self.other_token(ch, start),
        }?;
        Ok(self.cur_token)
    }

    fn special_op_token(&mut self, start: usize) -> Result<Token<'a>> {
        loop {
            match self.peek_one() {
                Some((_, _ch)) => {
                    if keyword::is_op(&(self.input[start..self.current() + 1].to_string())) {
                        self.next_one();
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }
        Ok(Token::Operator(
            &self.input[start..self.current()],
            Span(start, self.current()),
        ))
    }

    fn other_token(&mut self, ch: char, start: usize) -> Result<Token<'a>> {
        if self.try_parse_op(start) {
            return self.operator_token(start);
        }
        let (atom, start) = self.parse_var(start);
        if atom == "True" || atom == "true" {
            return self.bool_token(start, true);
        } else if atom == "False" || atom == "false" {
            return self.bool_token(start, false);
        }
        return self.function_or_reference_token(atom, start);
    }

    fn try_parse_op(&self, start: usize) -> bool {
        let mut tmp = self.clone();
        loop {
            match tmp.peek_one() {
                Some((_, ch)) => {
                    if is_whitespace_char(ch) || is_delim_char(ch) {
                        break;
                    }
                    tmp.next_one();
                }
                None => break,
            }
        }
        keyword::is_op(&tmp.input[start..tmp.current()])
    }

    fn operator_token(&mut self, start: usize) -> Result<Token<'a>> {
        loop {
            match self.peek_one() {
                Some((_, ch)) => {
                    if is_whitespace_char(ch) || is_delim_char(ch) {
                        break;
                    }
                    self.next_one();
                }
                None => break,
            }
        }
        return Ok(Token::Operator(
            self.input[start..self.current()].into(),
            Span(start, self.current()),
        ));
    }

    fn parse_var(&mut self, start: usize) -> (&'a str, usize) {
        loop {
            match self.peek_one() {
                Some((_, ch)) => {
                    if is_param_char(ch) {
                        self.next_one();
                        continue;
                    }
                    break;
                }
                None => break,
            }
        }
        (self.input[start..self.current()].into(), start)
    }

    pub fn peek(&self) -> Result<Token> {
        self.clone().next()
    }

    pub fn expect(&mut self, op: &str) -> Result<()> {
        let token = self.cur_token.clone();
        self.next()?;
        match token {
            Token::Delim(bracket, _) => {
                if bracket.string() == op {
                    return Ok(());
                }
            }
            Token::Operator(operator, _) => {
                if operator == op {
                    return Ok(());
                }
            }
            Token::Comma(c, _) => {
                if c == op {
                    return Ok(());
                }
            }
            _ => {
                return Err(Error::ExpectedOpNotExist(op.to_string()));
            }
        }
        Ok(())
    }

    fn delim_token(&mut self, start: usize) -> Result<Token<'a>> {
        Ok(Token::Delim(
            self.input[start..start + 1].into(),
            Span(start, start + 1),
        ))
    }

    fn comma_token(&mut self, start: usize) -> Result<Token<'a>> {
        Ok(Token::Comma(
            &self.input[start..start + 1],
            Span(start, start + 1),
        ))
    }

    fn semicolon_token(&mut self, start: usize) -> Result<Token<'a>> {
        Ok(Token::Semicolon(
            &self.input[start..start + 1],
            Span(start, start + 1),
        ))
    }

    fn number_token(&mut self, start: usize) -> Result<Token<'a>> {
        loop {
            match self.peek_one() {
                Some((_, ch)) => {
                    if (ch == '+' || ch == '-') && (self.cur_char != 'e' && self.cur_char != 'E') {
                        break;
                    }
                    if is_digit_char(ch) {
                        self.next_one();
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }
        match Decimal::from_str(&self.input[start..self.current()]) {
            Ok(val) => Ok(Token::Number(val, Span(start, self.current()))),
            Err(_) => Err(Error::InvalidNumber(
                self.input[start..self.current()].to_string(),
            )),
        }
    }

    fn function_or_reference_token(&self, atom: &'a str, start: usize) -> Result<Token<'a>> {
        let peek = self.peek()?;
        if peek.is_open_paren() {
            return Ok(Token::Function(atom, Span(start, self.current())));
        }
        Ok(Token::Reference(atom, Span(start, self.current())))
    }

    fn string_token(&mut self, start: usize) -> Result<Token<'a>> {
        let identifier = self.cur_char;
        let mut string_termmited = false;
        loop {
            match self.next_one() {
                Some((_, ch)) => {
                    if ch == identifier {
                        string_termmited = true;
                        break;
                    }
                }
                None => break,
            }
        }
        if !string_termmited {
            return Err(Error::UnterminatedString(self.current()));
        }
        Ok(Token::String(
            &self.input[start + 1..self.current() - 1],
            Span(start, self.current()),
        ))
    }

    fn bool_token(&mut self, start: usize, val: bool) -> Result<Token<'a>> {
        Ok(Token::Bool(val, Span(start, self.current())))
    }

    fn eat_whitespace(&mut self) -> Option<()> {
        loop {
            let (_, ch) = self.peek_one()?;
            if is_whitespace_char(ch) {
                self.next_one();
            } else {
                break;
            }
        }
        Some(())
    }

    fn current(&self) -> usize {
        self.chars
            .clone()
            .next()
            .map(|i| i.0)
            .unwrap_or_else(|| self.input.len())
    }
}

fn is_digit_char(ch: char) -> bool {
    return '0' <= ch && ch <= '9' || ch == '.' || ch == '-' || ch == 'e' || ch == 'E' || ch == '+';
}

fn is_whitespace_char(ch: char) -> bool {
    return ch == ' ' || ch == '\t' || ch == '\r' || ch == '\n';
}

fn is_delim_char(ch: char) -> bool {
    return ch == '(' || ch == ')' || ch == '[' || ch == ']' || ch == '{' || ch == '}';
}

fn is_param_char(ch: char) -> bool {
    return ('0' <= ch && ch <= '9')
        || ('a' <= ch && ch <= 'z')
        || ('A' <= ch && ch <= 'Z')
        || ch == '.'
        || ch == '_';
}

#[cfg(test)]
mod tests {
    use super::Tokenizer;
    use crate::init::init;
    use crate::token::DelimTokenType;
    use crate::token::Span;
    use crate::token::Token;
    use crate::token::Token::*;
    use rstest::rstest;
    use rust_decimal::prelude::*;

    #[rstest]
    #[case("true", true, 0, 4)]
    #[case(" True", true, 1, 5)]
    #[case(" \nfalse", false, 2, 7)]
    #[case(" \t False", false, 3, 8)]
    fn test_bool(
        #[case] input: &str,
        #[case] value: bool,
        #[case] start: usize,
        #[case] end: usize,
    ) {
        init();
        let mut tokenizer = Tokenizer::new(input);
        let ans = tokenizer.next().unwrap();
        assert_eq!(ans, Bool(value, Span(start, end)))
    }

    #[rstest]
    #[case(" 1234 ", "1234", 1, 5)]
    #[case(" 5.678 ", "5.678", 1, 6)]
    // #[case(" 10e-3 ", "10e-3", 1, 6)]
    // #[case(" 10e03 ", "10e03", 1, 6)]
    // #[case(" 2e+3 ", "2e+3", 1, 5)]
    fn test_number(
        #[case] input: &str,
        #[case] value: &str,
        #[case] start: usize,
        #[case] end: usize,
    ) {
        init();
        let mut tokenizer = Tokenizer::new(input);
        let ans = tokenizer.next().unwrap();
        assert_eq!(
            ans,
            Number(
                Decimal::from_str(value).unwrap_or_default(),
                Span(start, end)
            )
        )
    }

    #[rstest]
    #[case(" { ", DelimTokenType::OpenBrace, 1, 2)]
    #[case(" } ", DelimTokenType::CloseBrace, 1, 2)]
    #[case("  [", DelimTokenType::OpenBracket, 2, 3)]
    #[case("\n]", DelimTokenType::CloseBracket, 1, 2)]
    #[case(" ( ", DelimTokenType::OpenParen, 1, 2)]
    #[case("  \t)", DelimTokenType::CloseParen, 3, 4)]
    fn test_delim(
        #[case] input: &str,
        #[case] typ: DelimTokenType,
        #[case] start: usize,
        #[case] end: usize,
    ) {
        init();
        let mut tokenizer = Tokenizer::new(input);
        let ans = tokenizer.next().unwrap();
        assert_eq!(ans, Delim(typ, Span(start, end)))
    }

    #[rstest]
    #[case("", EOF)]
    #[case(" , ", Comma(",", Span(1, 2)))]
    #[case(" ; ", Semicolon(";", Span(1, 2)))]
    #[case(" +=", Operator("+=", Span(1, 3)))]
    #[case(" +=+", Operator("+=", Span(1, 3)))]
    #[case(" +=9", Operator("+=", Span(1, 3)))]
    #[case(" beginWith", Operator("beginWith", Span(1, 10)))]
    #[case(" endWith", Operator("endWith", Span(1, 8)))]
    fn test_other(#[case] input: &str, #[case] output: Token) {
        init();
        let mut tokenizer = Tokenizer::new(input);
        let ans = tokenizer.next().unwrap();
        assert_eq!(ans, output);
    }

    #[rstest]
    #[case(" 'dsfasdfdsa' ", "dsfasdfdsa", 1, 13)]
    #[case("\"dffd\"", "dffd", 0, 6)]
    fn test_string(
        #[case] input: &str,
        #[case] value: &str,
        #[case] start: usize,
        #[case] end: usize,
    ) {
        init();
        let mut tokenizer = Tokenizer::new(input);
        let ans = tokenizer.next().unwrap();
        assert_eq!(ans, String(value, Span(start, end)));
    }

    #[rstest]
    #[case(" d09f_5 ", Reference("d09f_5", Span(1, 7)))]
    #[case(" d09f_5() ", Function("d09f_5", Span(1, 7)))]
    #[case(" d09f_>", Reference("d09f_", Span(1, 6)))]
    fn test_reference_function(#[case] input: &str, #[case] output: Token) {
        init();
        let mut tokenizer = Tokenizer::new(input);
        let ans = tokenizer.next().unwrap();
        assert_eq!(ans, output);
    }

    #[rstest]
    #[case("\"jajd'")]
    #[case("0e.3")]
    fn test_err(#[case] input: &str) {
        init();
        let mut tokenizer = Tokenizer::new(input);
        let ans = tokenizer.next();
        assert!(ans.is_err())
    }
}
