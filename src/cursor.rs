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

    /// Advances the cursor while the predicate returns true for the peeked
    /// character. Returns the number of times the cursor was advanced.
    pub fn advance_while(&mut self, predicate: impl Fn(char) -> bool) -> usize {
        let mut count = 0;

        while let Some(char) = self.peek() {
            if !predicate(char) {
                break;
            }

            self.advance();
            count += 1;
        }

        count
    }
}
