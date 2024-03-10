use crate::{
    cursor::Cursor,
    error::LexerError,
    token::{Span, Token, TokenKind},
};

use anyhow::{bail, Result};

/// Takes in a stream of characters and transforms it into
/// a sequence of `Token`s, which can be used for parsing
pub struct Lexer<'a> {
    cursor: Cursor<'a>,

    input: &'a str,
    file: &'static str,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str, file: &'static str) -> Self {
        Self {
            cursor: Cursor::new(input),
            input,
            file,
        }
    }

    pub fn tokenize(mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token() {
            tokens.push(token?);
        }

        Ok(tokens
            .into_iter()
            .filter(|token| token.kind != TokenKind::Whitespace)
            .collect())
    }

    fn next_token(&mut self) -> Option<Result<Token>> {
        let start = self.cursor.position();

        let kind = match self.cursor.advance()? {
            c if c.is_whitespace() => {
                self.cursor.advance_while(|c| c.is_whitespace());
                Ok(TokenKind::Whitespace)
            }

            '0'..='9' => self.parse_number(),

            _ => panic!("uwu"),
        };

        let end = self.cursor.position();

        Some(kind.map(|kind| Token {
            span: (start..end, self.file).into(),
            kind,
        }))
    }

    fn parse_number(&mut self) -> Result<TokenKind> {
        let start = self.cursor.position() - 1;

        self.cursor.advance_while(|c| c.is_ascii_digit());

        if let Some('.') = self.cursor.peek() {
            let decimal_places = self.cursor.advance_while(|c| c == '.');

            self.cursor.advance_while(|c| c.is_ascii_digit());

            let end = self.cursor.position();
            let span = Span::from((start..end, self.file));

            let literal = self.input[span].to_string();

            if decimal_places != 1 {
                bail!(LexerError::InvalidNumberLiteral(span, literal));
            }

            return Ok(TokenKind::Float(
                self.input[start..self.cursor.position()].parse().unwrap(),
            ));
        }

        let end = self.cursor.position();

        Ok(TokenKind::Integer(self.input[start..end].parse().unwrap()))
    }
}
