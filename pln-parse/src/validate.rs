use crate::ast::{Item, Node, Unit};
use crate::error::{ErrorKind, ParseError};

/// Validate that col/row units are used in the correct split direction.
pub fn validate(item: &Item) -> Result<(), ParseError> {
    validate_node(&item.node)
}

fn validate_node(node: &Node) -> Result<(), ParseError> {
    match node {
        Node::Panel { .. } => Ok(()),
        Node::HSplit { children } => {
            for child in children {
                check_unit(child, "horizontal (|)", Unit::Row, "row")?;
                validate_node(&child.node)?;
            }
            Ok(())
        }
        Node::VSplit { children } => {
            for child in children {
                check_unit(child, "vertical (/)", Unit::Col, "col")?;
                validate_node(&child.node)?;
            }
            Ok(())
        }
    }
}

fn check_unit(
    item: &Item,
    split_direction: &str,
    invalid_unit: Unit,
    unit_name: &str,
) -> Result<(), ParseError> {
    if let Some(ref size) = item.size
        && size.unit == invalid_unit
    {
        return Err(ParseError {
            message: format!(
                "'{}' unit is not valid in a {} split",
                unit_name, split_direction
            ),
            span: item.span,
            kind: ErrorKind::InvalidUnitForSplit {
                unit: unit_name.to_string(),
                split: split_direction.to_string(),
            },
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::parser;

    use super::*;

    fn parse_and_validate(input: &str) -> Result<(), ParseError> {
        let item = parser::parse(input).unwrap();
        validate(&item)
    }

    #[test]
    fn col_in_hsplit_is_valid() {
        assert!(parse_and_validate("(sidebar=80col|main)").is_ok());
    }

    #[test]
    fn row_in_vsplit_is_valid() {
        assert!(parse_and_validate("(header=5row/body)").is_ok());
    }

    #[test]
    fn col_in_vsplit_is_invalid() {
        assert!(parse_and_validate("(header=80col/body)").is_err());
    }

    #[test]
    fn row_in_hsplit_is_invalid() {
        assert!(parse_and_validate("(sidebar=5row|main)").is_err());
    }

    #[test]
    fn nested_validation() {
        assert!(parse_and_validate("(editor|(header=80col/body))").is_err());
    }

    #[test]
    fn fr_px_percent_always_valid() {
        assert!(parse_and_validate("(a=2fr|b=200px|c=30%)").is_ok());
        assert!(parse_and_validate("(a=2fr/b=200px/c=30%)").is_ok());
    }
}
