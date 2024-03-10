use std::ops::{Index, Range};

/// A span of text
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,

    pub file: &'static str,
}

impl Span {
    /// Offset this span by some size
    pub fn offset(&self, by: isize) -> Self {
        Self {
            start: self.start.saturating_add_signed(by),
            end: self.end.saturating_add_signed(by),
            file: self.file,
        }
    }
}

/// A token within a file that's being parsed
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// The position offset from the start of the file
    pub span: Span,
    /// The kind of token
    pub kind: TokenKind,
}

/// A kind of token
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind {
    Float(f64),
    Integer(usize),
    Whitespace,
}

impl From<(Range<usize>, &'static str)> for Span {
    fn from(value: (Range<usize>, &'static str)) -> Self {
        Self {
            start: value.0.start,
            end: value.0.end,
            file: value.1,
        }
    }
}

impl Index<Span> for str {
    type Output = str;

    fn index(&self, index: Span) -> &Self::Output {
        &self[index.start..index.end]
    }
}
