pub mod ast;
pub mod error;
pub mod parser;
pub mod validate;

pub use ast::{Item, Node, Size, Span, Unit};
pub use error::ParseError;

/// Parse a PLN layout string into an AST.
pub fn parse(input: &str) -> Result<Item, ParseError> {
    parser::parse(input)
}

/// Parse and validate a PLN layout string.
///
/// This runs both parsing and semantic validation (e.g. checking that
/// col/row units are used in the correct split direction).
pub fn parse_and_validate(input: &str) -> Result<Item, ParseError> {
    let item = parser::parse(input)?;
    validate::validate(&item)?;
    Ok(item)
}
