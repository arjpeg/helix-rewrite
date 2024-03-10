use anyhow::Error;
use owo_colors::OwoColorize;
use thiserror::Error;

use crate::token::Span;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("Encountered an unknown symbol, '{}'", .1)]
    InvalidSymbol(Span, String),
    #[error("Number literal {} had too many decimal places", .1)]
    InvalidNumberLiteral(Span, String),
}

pub fn print_error(error: Error, source: &'static str) {
    let span = match error.downcast_ref::<LexerError>() {
        Some(error) => match error {
            LexerError::InvalidSymbol(span, _) => span,
            LexerError::InvalidNumberLiteral(span, _) => span,
        },
        None => panic!(),
    };

    let upto_start = &source[..span.start];

    let line_number = upto_start.matches('\n').count();
    let line = source.lines().nth(line_number).unwrap();

    let line_start_index = upto_start.rfind('\n').unwrap_or(0);
    let line_range = span.offset(-(line_start_index as isize));

    let file_format = format!("  {}:{}  ", span.file, line_number + 1);

    dbg!(&(file_format.len() + line_range.start));

    let error_arrows = format!(
        "{}{}",
        " ".repeat(file_format.len() + line_range.start),
        "^".repeat(line_range.end - line_range.start),
    );

    eprintln!("{}: {}", "Error".red().bold(), error.bold());
    eprintln!("{}{line}", file_format.white().dimmed());
    eprintln!("{}", error_arrows);
}
