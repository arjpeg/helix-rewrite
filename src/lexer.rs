use core::panic;
use std::{iter::Peekable, str::Chars};

use crate::{
    cursor::Cursor,
    error::Result,
    token::{Token, TokenKind},
};

/// Takes in a stream of characters and transforms it into
/// a sequence of `Token`s, which can be used for parsing
pub struct Lexer<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            cursor: Cursor::new(input),
        }
    }

    pub fn tokenize(mut self) {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token() {
            tokens.push(token)
        }
    }

    fn next_token(&mut self) -> Option<Result<Token>> {
        let start = self.cursor.position();

        let token_kind = match self.cursor.advance()? {
            '0'..='9' => {
                println!("number");
                self.parse_number()
            }
            _ => panic!("uwu"),
        };

        todo!()
    }

    fn parse_number(&mut self) -> Result<TokenKind> {
        let mut dots = 0;

        self.cursor.advance_while(|c| c.is_ascii_digit());

        println!("here {:?}", self.cursor.current());
        todo!()
    }
}
