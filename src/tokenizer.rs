use std::str;
use crate::token::{Span, Token};
use rust_decimal::prelude::*;
use crate::error::Error;
use crate::define::Result;

pub struct Tokenizer<'a> {
    input: &'a str,
    chars: str::CharIndices<'a>,
    cur_char: char
}

impl <'a> Tokenizer<'a> {

    pub fn new(input: &str) -> Tokenizer {
        Tokenizer{
            input: input,
            chars: input.char_indices(),
            cur_char: ' ',
        }
    }

    fn next_one(&mut self) -> Option<(usize, char)> {
        let (cur, cur_char) = self.chars.next()?;
        self.cur_char = cur_char;
        Some((cur, cur_char))
    }

    fn peek_one(&mut self) -> Option<(usize,char)> {
        Some(self.chars.clone().next()?)
    }

    pub fn next(&mut self) -> Result<Option<Token>> {
        self.eat_whitespace();
        match self.next_one() {
            // Some((start, ',')) => Ok(Some(self.comma_token(start)?)),
            Some((start, '+' | '-' | '*' | '/' | '%' | '(' | ')')) => self.operator_token(start),
            Some((start, _ch @'0' ..= '9')) => self.literal_token(start),
            Some((start, '&')) => self.refrence_token(start),
            Some((start, '$')) => self.function_token(start),
            Some((start, 't')) => self.bool_token(start, true),
            Some((start, 'f')) => self.bool_token(start, false),
            Some((start, '"')) => self.string_token(start),
            Some((start, ',')) => self.comma_token(start),
            None => Ok(None),
            Some((start, ch)) => Err(Error::NotSupportedChar(start, ch)),
        }
    }

    fn comma_token(&mut self, start: usize) -> Result<Option<Token>> {
        Ok(Some(Token::Comma(self.input[start..start+1].to_owned(), Span(start, start+1))))
    }

    fn operator_token(&mut self, start: usize) -> Result<Option<Token>> {
        Ok(Some(Token::Operator(self.input[start..start+1].to_owned(), Span(start, start+1))))
    }

    fn literal_token(&mut self, start: usize) -> Result<Option<Token>> {
        'outer: loop {
            match self.peek_one() {
                Some((_, ch)) => {
                    if (ch == '+' || ch == '-') && (self.cur_char != 'e' || self.cur_char != 'E') {
                        break 'outer
                    }
                    if is_digit_char(ch) {
                        self.next_one();
                    } else {
                        break 'outer
                    }
                },
                None => break 'outer,
            }
        }
        match rust_decimal::Decimal::from_str(&self.input[start..self.current()]) {
            Ok(val) => Ok(Some(Token::Literal(val, Span(start, self.current())))),
            Err(_) => Err(Error::InvalidNumber(self.input[start..self.current()].to_string()))
        }
    }

    fn refrence_token(&mut self, start: usize) -> Result<Option<Token>> {
        'outer: loop {
            match self.peek_one() {
                Some((_, ch)) => {
                    if is_param_char(ch) {
                        self.next_one();
                    } else {
                        break 'outer
                    }
                }
                None => break 'outer,
            }
        }
        if self.current() - start <= 1 {
            return Err(Error::UnexpectedEOF(self.current()));
        }
        Ok(Some(Token::Reference(self.input[start+1..self.current()].to_owned(), Span(start, self.current()))))
    }

    fn function_token(&mut self, start: usize) -> Result<Option<Token>> {
        loop {
            match self.peek_one() {
                Some((_, ch)) => {
                    if is_param_char(ch) {
                        self.next_one();
                    } else {
                        break
                    }
                },
                None => break
            }
        }
        Ok(Some(Token::Function(self.input[start+1..self.current()].to_owned(), Span(start, self.current()))))
    }

    fn string_token(&mut self, start: usize) -> Result<Option<Token>> {
        'outer: loop {
            match self.peek_one() {
                Some((_, ch)) => {
                    if ch != '"' {
                        self.next_one();
                    } else {
                        break 'outer
                    }
                }
                None => break 'outer
            }
        }
        match self.peek_one() {
            Some((_, '"')) => {
                self.next_one();
            },
            _ => return Err(Error::UnexpectedEOF(self.current()))
        }
        Ok(Some(Token::String(self.input[start+1..self.current()-1].to_owned(), Span(start, self.current()))))
    }

    fn bool_token(&mut self, start: usize, val: bool) -> Result<Option<Token>> {
        if (val && self.parse_ident("rue")) || (!val && self.parse_ident("alse")) {
            Ok(Some(Token::Bool(val, Span(start, self.current()))))
        } else {
            Err(Error::InvalidBool(self.current()))
        }
    }

    fn parse_ident(&mut self, expected: &str) -> bool {
        for (_, expect) in expected.char_indices() {
            match self.next_one() {
                Some((_, ch)) => {
                    if ch != expect {
                        return false
                    }
                },
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
    return '0' <= ch && ch <= '9' || ch == '.' || ch =='-' || ch =='e' || ch =='E' || ch == '+'
}

fn is_whitespace_char(ch: char) -> bool {
    return ch == ' ' || ch == '\t' || ch == '\r' || ch == '\n'
}

fn is_param_char(ch: char) -> bool {
    return ('0' <= ch && ch <= '9') ||
        ('a' <= ch && ch <= 'z') ||
        ('A' <= ch && ch <= 'Z') ||
        ch == '.' ||
        ch == '_'
}

#[test]
fn test() {
    let input = "$ROA($test(&mm, \"[1,2,3]\"))";
    let mut tokenizer = Tokenizer::new(input);
    loop {
        match tokenizer.next() {
            Ok(t) => {
                if t.is_some() {
                    println!("{}", t.unwrap())
                } else {
                    break
                }
            },
            Err(e) => println!("{}", e)
        }
    }
}