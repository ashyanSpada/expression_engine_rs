use std::error::Error;

pub struct Tokenizer<'a> {
    input: &'a str,
    cur: i32
}

pub fn new<'a>(input: &'a str)->Tokenizer {
    Tokenizer { input: input, cur: -1 }
}

impl <'a> Tokenizer<'a> {
    pub fn next_byte(&mut self) -> Option<u8> {
            self.cur += 1;
            if self.cur < self.input.len() as i32 {
                return Some(self.input[self.cur as usize]);
            }
            None
        }
        pub fn peak_byte(&self) -> Option<u8> {

        }
}