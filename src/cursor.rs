use std::{iter::Peekable, str::Chars};

pub struct Cursor<'a> {
    chars: Peekable<Chars<'a>>,
    position: usize,
    current: Option<char>,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
            position: 0,
            current: None,
        }
    }

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn current(&self) -> Option<char> {
        self.current
    }

    pub fn peek(&mut self) -> Option<char> {
        self.chars.peek().copied()
    }

    pub fn advance(&mut self) -> Option<char> {
        let char = self.chars.next();

        self.position += char.map(|c| c.len_utf8()).unwrap_or(0);
        self.current = char;

        char
    }

    pub fn advance_while(&mut self, predicate: impl Fn(char) -> bool) {
        while let Some(char) = self.advance() {
            if !predicate(char) {
                break;
            }

            self.advance();
        }
    }
}
