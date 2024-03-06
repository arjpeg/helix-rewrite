use std::ops::Range;

/// A span of text
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub file: &'static str,
    pub range: Range<usize>,
}

/// A token within a file that's being parsed
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
}
