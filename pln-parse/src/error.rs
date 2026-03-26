use std::fmt;

use crate::ast::Span;

/// A parse or validation error with source location.
#[derive(Debug, Clone)]
pub struct ParseError {
    pub message: String,
    pub span: Span,
    pub kind: ErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorKind {
    UnexpectedChar(char),
    UnexpectedEof,
    MixedOperators,
    UnclosedGroup,
    UnclosedString,
    InvalidUnit(String),
    InvalidNumber,
    EmptyGroup,
    TrailingInput,
    InvalidUnitForSplit { unit: String, split: String },
}

impl ParseError {
    pub fn format_with_source(&self, source: &str) -> String {
        let (line, column) = byte_offset_to_position(source, self.span.start);
        let source_line = source.lines().nth(line - 1).unwrap_or("");

        let mut output = format!("error: {}\n", self.message);
        output.push_str(&format!("  --> input:{}:{}\n", line, column));
        output.push_str("   |\n");
        output.push_str(&format!("{:>3} | {}\n", line, source_line));
        output.push_str("   | ");

        // Caret pointing at the error position
        let line_start = source[..self.span.start]
            .rfind('\n')
            .map(|position| position + 1)
            .unwrap_or(0);
        let offset_in_line = self.span.start - line_start;
        for _ in 0..offset_in_line {
            output.push(' ');
        }
        let caret_len = (self.span.end - self.span.start).max(1);
        for _ in 0..caret_len {
            output.push('^');
        }
        output.push('\n');
        output
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.message)
    }
}

impl std::error::Error for ParseError {}

fn byte_offset_to_position(source: &str, offset: usize) -> (usize, usize) {
    let before = &source[..offset.min(source.len())];
    let line = before
        .chars()
        .filter(|&character| character == '\n')
        .count()
        + 1;
    let column = before
        .rfind('\n')
        .map(|position| offset - position)
        .unwrap_or(offset + 1);
    (line, column)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn byte_offset_single_line() {
        assert_eq!(byte_offset_to_position("(a|b/c)", 4), (1, 5));
    }

    #[test]
    fn byte_offset_multiline() {
        assert_eq!(byte_offset_to_position("abc\ndef\nghi", 5), (2, 2));
    }
}
