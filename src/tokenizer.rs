use crate::define::Result;
use crate::error::Error;
use crate::keyword::{KeywordManager, KeywordType};
use crate::token::{Span, Token};
use rust_decimal::prelude::*;
use std::str;

#[derive(Clone)]
pub struct Tokenizer<'a> {
    input: &'a str,
    chars: str::CharIndices<'a>,
    cur_char: char,
    pub cur_token: Token,
    pub prev_token: Token,
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

    pub fn next(&mut self) -> Result<Token> {
        self.eat_whitespace();
        self.prev_token = self.cur_token.clone();
        self.cur_token = match self.next_one() {
            Some((
                start,
                '+' | '-' | '*' | '/' | '%' | '&' | '!' | '=' | '|' | '>' | '<' | '?' | ':',
            )) => self.operator_token(start),
            Some((start, '(' | ')' | '[' | ']' | '{' | '}')) => self.delim_token(start),
            Some((start, _ch @ '0'..='9')) => self.number_token(start),
            Some((start, '"' | '\'')) => self.string_token(start),
            Some((start, ';')) => self.semicolon_token(start),
            Some((start, ',')) => self.comma_token(start),
            None => Ok(Token::EOF),
            Some((start, ch)) => self.other_token(ch, start),
        }?;
        Ok(self.cur_token.clone())
    }

    fn other_token(&mut self, ch: char, start: usize) -> Result<Token> {
        if (ch == 't' && self.try_parse_ident("rue")) || (ch == 'f' && self.try_parse_ident("alse"))
        {
            return self.bool_token(start, ch == 't');
        }
        if is_param_char(ch) {
            return self.op_reference_function_token(start);
        }
        Err(Error::NotSupportedChar(start, ch))
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

    fn delim_token(&mut self, start: usize) -> Result<Token> {
        Ok(Token::Delim(
            self.input[start..start + 1].into(),
            Span(start, start + 1),
        ))
    }

    fn comma_token(&mut self, start: usize) -> Result<Token> {
        Ok(Token::Comma(
            self.input[start..start + 1].to_owned(),
            Span(start, start + 1),
        ))
    }

    fn semicolon_token(&mut self, start: usize) -> Result<Token> {
        Ok(Token::Semicolon(
            self.input[start..start + 1].to_owned(),
            Span(start, start + 1),
        ))
    }

    fn operator_token(&mut self, start: usize) -> Result<Token> {
        loop {
            match self.peek_one() {
                Some((_, ch)) => {
                    if is_operator_char(ch) {
                        self.next_one();
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }
        Ok(Token::Operator(
            self.input[start..self.current()].to_owned(),
            Span(start, self.current()),
        ))
    }

    fn number_token(&mut self, start: usize) -> Result<Token> {
        loop {
            match self.peek_one() {
                Some((_, ch)) => {
                    if (ch == '+' || ch == '-') && (self.cur_char != 'e' || self.cur_char != 'E') {
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
        match rust_decimal::Decimal::from_str(&self.input[start..self.current()]) {
            Ok(val) => Ok(Token::Number(val, Span(start, self.current()))),
            Err(_) => Err(Error::InvalidNumber(
                self.input[start..self.current()].to_string(),
            )),
        }
    }

    fn op_reference_function_token(&mut self, start: usize) -> Result<Token> {
        loop {
            match self.peek_one() {
                Some((_, ch)) => {
                    if is_param_char(ch) {
                        self.next_one();
                    } else {
                        break;
                    }
                }
                None => break,
            }
        }
        let name = self.input[start..self.current()].to_string();
        match KeywordManager::new().get_type(&name) {
            KeywordType::Function => Ok(Token::Function(name.clone(), Span(start, self.current()))),
            KeywordType::Op => Ok(Token::Operator(name.clone(), Span(start, self.current()))),
            KeywordType::Reference => {
                Ok(Token::Reference(name.clone(), Span(start, self.current())))
            }
            KeywordType::Unknown => {
                let token = self.peek()?;
                if token.is_left_paren() {
                    return Ok(Token::Function(
                        self.input[start..self.current()].to_owned(),
                        Span(start, self.current()),
                    ));
                }
                return Ok(Token::Reference(
                    self.input[start..self.current()].to_owned(),
                    Span(start, self.current()),
                ));
            }
        }
    }

    fn string_token(&mut self, start: usize) -> Result<Token> {
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
            self.input[start + 1..self.current() - 1].to_owned(),
            Span(start, self.current()),
        ))
    }

    fn bool_token(&mut self, start: usize, val: bool) -> Result<Token> {
        if (val && self.parse_ident("rue")) || (!val && self.parse_ident("alse")) {
            Ok(Token::Bool(val, Span(start, self.current())))
        } else {
            Err(Error::InvalidBool(self.current()))
        }
    }

    fn try_parse_ident(&self, expected: &str) -> bool {
        self.clone().parse_ident(expected)
    }

    fn parse_ident(&mut self, expected: &str) -> bool {
        for (_, expect) in expected.char_indices() {
            match self.next_one() {
                Some((_, ch)) => {
                    if ch != expect {
                        return false;
                    }
                }
                None => return false,
            }
        }
        true
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

fn is_param_char(ch: char) -> bool {
    return ('0' <= ch && ch <= '9')
        || ('a' <= ch && ch <= 'z')
        || ('A' <= ch && ch <= 'Z')
        || ch == '.'
        || ch == '_';
}

fn is_operator_char(ch: char) -> bool {
    match ch {
        '+' | '-' | '*' | '/' | '%' | '&' | '!' | '=' | '|' | '>' | '<' | '?' | ':' => true,
        _ => false,
    }
}

#[test]
fn test() {
    let input = "{1:2+3*2};";
    let mut tokenizer = Tokenizer::new(input);
    loop {
        match tokenizer.next() {
            Ok(Token::EOF) => break,
            Ok(t) => {
                println!("{}", t)
            }
            Err(e) => println!("{}", e),
        }
    }
}
