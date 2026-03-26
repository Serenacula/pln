use crate::ast::{Item, Node, Size, Span, Unit};
use crate::error::{ErrorKind, ParseError};

pub fn parse(input: &str) -> Result<Item, ParseError> {
    let mut parser = Parser { input, pos: 0 };
    let item = parser.parse_item()?;
    parser.skip_whitespace();
    if parser.pos < parser.input.len() {
        return Err(parser.error(ErrorKind::TrailingInput, "unexpected input after layout"));
    }
    Ok(item)
}

struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

impl<'a> Parser<'a> {
    fn parse_item(&mut self) -> Result<Item, ParseError> {
        self.skip_whitespace();
        let start = self.pos;

        match self.peek() {
            Some('(') => {
                // Groups may carry a size from inside (single-item) or outside (split)
                let mut item = self.parse_group()?;
                // A group can also have an outer size: ((a|b)=3fr / c)
                if let Some(outer_size) = self.try_parse_size()? {
                    item.size = Some(outer_size);
                }
                item.span.start = start;
                item.span.end = self.pos;
                Ok(item)
            }
            Some(_) => {
                let node = self.parse_panel()?;
                let size = self.try_parse_size()?;
                let span = Span {
                    start,
                    end: self.pos,
                };
                Ok(Item { node, size, span })
            }
            None => Err(self.error(ErrorKind::UnexpectedEof, "expected a panel or group")),
        }
    }

    fn parse_group(&mut self) -> Result<Item, ParseError> {
        let group_start = self.pos;
        self.advance(); // consume '('

        self.skip_whitespace();
        if self.peek() == Some(')') {
            return Err(ParseError {
                message: "empty group".into(),
                span: Span {
                    start: group_start,
                    end: self.pos + 1,
                },
                kind: ErrorKind::EmptyGroup,
            });
        }

        let first = self.parse_item()?;
        self.skip_whitespace();

        // Check for operator
        match self.peek() {
            Some(')') => {
                // Single-item group — collapse to the inner item (preserving its size)
                self.advance();
                Ok(first)
            }
            Some('|') => {
                let node = self.parse_split_tail(first, '|', group_start)?;
                Ok(Item {
                    node,
                    size: None,
                    span: Span {
                        start: group_start,
                        end: self.pos,
                    },
                })
            }
            Some('/') => {
                let node = self.parse_split_tail(first, '/', group_start)?;
                Ok(Item {
                    node,
                    size: None,
                    span: Span {
                        start: group_start,
                        end: self.pos,
                    },
                })
            }
            Some(_) => Err(self.error(
                ErrorKind::UnexpectedChar(self.input[self.pos..].chars().next().unwrap()),
                "expected '|', '/', or ')'",
            )),
            None => Err(ParseError {
                message: "unclosed group — expected ')'".into(),
                span: Span {
                    start: group_start,
                    end: self.pos,
                },
                kind: ErrorKind::UnclosedGroup,
            }),
        }
    }

    fn parse_split_tail(
        &mut self,
        first: Item,
        operator: char,
        group_start: usize,
    ) -> Result<Node, ParseError> {
        let mut children = vec![first];

        while self.peek() == Some(operator) {
            self.advance(); // consume operator
            children.push(self.parse_item()?);
            self.skip_whitespace();
        }

        // Check for mixed operators
        self.skip_whitespace();
        let other_operator = if operator == '|' { '/' } else { '|' };
        if self.peek() == Some(other_operator) {
            return Err(self.error(
                ErrorKind::MixedOperators,
                &format!(
                    "mixed operators — group uses '{}' but found '{}'",
                    operator, other_operator
                ),
            ));
        }

        match self.peek() {
            Some(')') => {
                self.advance();
            }
            Some(_) => {
                return Err(self.error(
                    ErrorKind::UnexpectedChar(self.input[self.pos..].chars().next().unwrap()),
                    "expected operator or ')'",
                ));
            }
            None => {
                return Err(ParseError {
                    message: "unclosed group — expected ')'".into(),
                    span: Span {
                        start: group_start,
                        end: self.pos,
                    },
                    kind: ErrorKind::UnclosedGroup,
                });
            }
        }

        match operator {
            '|' => Ok(Node::HSplit { children }),
            '/' => Ok(Node::VSplit { children }),
            _ => unreachable!(),
        }
    }

    fn parse_panel(&mut self) -> Result<Node, ParseError> {
        self.skip_whitespace();
        match self.peek() {
            Some('"') | Some('\'') => self.parse_quoted(),
            Some(_) => self.parse_word(),
            None => Err(self.error(ErrorKind::UnexpectedEof, "expected a panel name")),
        }
    }

    fn parse_word(&mut self) -> Result<Node, ParseError> {
        let start = self.pos;
        while let Some(character) = self.peek() {
            if character.is_whitespace() || "|/()=\"'".contains(character) {
                break;
            }
            self.advance();
        }
        if self.pos == start {
            return Err(self.error(ErrorKind::UnexpectedEof, "expected a panel name"));
        }
        let name = self.input[start..self.pos].to_string();
        Ok(Node::Panel { name })
    }

    fn parse_quoted(&mut self) -> Result<Node, ParseError> {
        let start = self.pos;
        let quote = self.advance();
        let mut name = String::new();

        loop {
            match self.peek() {
                None => {
                    return Err(ParseError {
                        message: format!("unclosed string — expected closing {}", quote),
                        span: Span {
                            start,
                            end: self.pos,
                        },
                        kind: ErrorKind::UnclosedString,
                    });
                }
                Some('\\') => {
                    self.advance();
                    match self.peek() {
                        Some(character) if character == quote => {
                            name.push(character);
                            self.advance();
                        }
                        Some(character) => {
                            name.push('\\');
                            name.push(character);
                            self.advance();
                        }
                        None => {
                            return Err(ParseError {
                                message: format!("unclosed string — expected closing {}", quote),
                                span: Span {
                                    start,
                                    end: self.pos,
                                },
                                kind: ErrorKind::UnclosedString,
                            });
                        }
                    }
                }
                Some(character) if character == quote => {
                    self.advance();
                    return Ok(Node::Panel { name });
                }
                Some(character) => {
                    name.push(character);
                    self.advance();
                }
            }
        }
    }

    fn try_parse_size(&mut self) -> Result<Option<Size>, ParseError> {
        self.skip_whitespace();
        if self.peek() != Some('=') {
            return Ok(None);
        }
        self.advance(); // consume '='
        self.skip_whitespace();

        let number_start = self.pos;
        let value = self.parse_number()?;
        let unit = self.parse_unit(number_start)?;

        Ok(Some(Size { value, unit }))
    }

    fn parse_number(&mut self) -> Result<f64, ParseError> {
        let start = self.pos;

        // Integer part
        while let Some(character) = self.peek() {
            if character.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        // Decimal part
        if self.peek() == Some('.') {
            self.advance();
            while let Some(character) = self.peek() {
                if character.is_ascii_digit() {
                    self.advance();
                } else {
                    break;
                }
            }
        }

        if self.pos == start {
            return Err(self.error(ErrorKind::InvalidNumber, "expected a number"));
        }

        self.input[start..self.pos]
            .parse::<f64>()
            .map_err(|_| self.error_at(start, ErrorKind::InvalidNumber, "invalid number"))
    }

    fn parse_unit(&mut self, value_start: usize) -> Result<Unit, ParseError> {
        let unit_start = self.pos;

        // Special case: %
        if self.peek() == Some('%') {
            self.advance();
            return Ok(Unit::Percent);
        }

        // Read alphabetic unit name
        while let Some(character) = self.peek() {
            if character.is_ascii_alphabetic() {
                self.advance();
            } else {
                break;
            }
        }

        let unit_str = &self.input[unit_start..self.pos];
        match unit_str {
            "fr" => Ok(Unit::Fr),
            "col" => Ok(Unit::Col),
            "row" => Ok(Unit::Row),
            "px" => Ok(Unit::Px),
            "" => Err(ParseError {
                message: "size value requires a unit (fr, col, row, px, %)".into(),
                span: Span {
                    start: value_start,
                    end: self.pos,
                },
                kind: ErrorKind::InvalidUnit(String::new()),
            }),
            other => Err(ParseError {
                message: format!("unknown unit '{}' — expected fr, col, row, px, or %", other),
                span: Span {
                    start: unit_start,
                    end: self.pos,
                },
                kind: ErrorKind::InvalidUnit(other.to_string()),
            }),
        }
    }

    // -- Helpers --

    fn peek(&self) -> Option<char> {
        self.input[self.pos..].chars().next()
    }

    fn advance(&mut self) -> char {
        let character = self.input[self.pos..].chars().next().unwrap();
        self.pos += character.len_utf8();
        character
    }

    fn skip_whitespace(&mut self) {
        while let Some(character) = self.peek() {
            if character.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    fn error(&self, kind: ErrorKind, message: &str) -> ParseError {
        self.error_at(self.pos, kind, message)
    }

    fn error_at(&self, position: usize, kind: ErrorKind, message: &str) -> ParseError {
        ParseError {
            message: message.to_string(),
            span: Span {
                start: position,
                end: (position + 1).min(self.input.len()),
            },
            kind,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_ok(input: &str) -> Item {
        parse(input).unwrap_or_else(|error| panic!("parse failed for '{}': {}", input, error))
    }

    fn parse_err(input: &str) -> ParseError {
        parse(input).unwrap_err()
    }

    // -- Basic panels --

    #[test]
    fn bare_panel() {
        let item = parse_ok("editor");
        assert!(matches!(item.node, Node::Panel { ref name } if name == "editor"));
        assert!(item.size.is_none());
    }

    #[test]
    fn double_quoted_panel() {
        let item = parse_ok("\"Left Panel\"");
        assert!(matches!(item.node, Node::Panel { ref name } if name == "Left Panel"));
    }

    #[test]
    fn single_quoted_panel() {
        let item = parse_ok("'Right Panel'");
        assert!(matches!(item.node, Node::Panel { ref name } if name == "Right Panel"));
    }

    #[test]
    fn quoted_with_escape() {
        let item = parse_ok(r#""it\"s a panel""#);
        assert!(matches!(item.node, Node::Panel { ref name } if name == "it\"s a panel"));
    }

    #[test]
    fn single_quoted_with_escape() {
        let item = parse_ok(r"'it\'s a panel'");
        assert!(matches!(item.node, Node::Panel { ref name } if name == "it's a panel"));
    }

    // -- Splits --

    #[test]
    fn horizontal_split() {
        let item = parse_ok("(left|right)");
        match &item.node {
            Node::HSplit { children } => {
                assert_eq!(children.len(), 2);
                assert!(matches!(&children[0].node, Node::Panel { name } if name == "left"));
                assert!(matches!(&children[1].node, Node::Panel { name } if name == "right"));
            }
            other => panic!("expected HSplit, got {:?}", other),
        }
    }

    #[test]
    fn vertical_split() {
        let item = parse_ok("(top/bottom)");
        match &item.node {
            Node::VSplit { children } => {
                assert_eq!(children.len(), 2);
                assert!(matches!(&children[0].node, Node::Panel { name } if name == "top"));
                assert!(matches!(&children[1].node, Node::Panel { name } if name == "bottom"));
            }
            other => panic!("expected VSplit, got {:?}", other),
        }
    }

    #[test]
    fn three_way_split() {
        let item = parse_ok("(a|b|c)");
        match &item.node {
            Node::HSplit { children } => assert_eq!(children.len(), 3),
            other => panic!("expected HSplit, got {:?}", other),
        }
    }

    // -- Single-item groups --

    #[test]
    fn single_item_group() {
        let item = parse_ok("(panel)");
        assert!(matches!(item.node, Node::Panel { ref name } if name == "panel"));
    }

    #[test]
    fn sized_single_item_group() {
        let item = parse_ok("(panel=2fr)");
        assert!(matches!(item.node, Node::Panel { ref name } if name == "panel"));
        let size = item.size.unwrap();
        assert_eq!(size.value, 2.0);
        assert_eq!(size.unit, Unit::Fr);
    }

    #[test]
    fn redundant_parens() {
        let item = parse_ok("((left)|right)");
        match &item.node {
            Node::HSplit { children } => {
                assert_eq!(children.len(), 2);
                assert!(matches!(&children[0].node, Node::Panel { name } if name == "left"));
            }
            other => panic!("expected HSplit, got {:?}", other),
        }
    }

    // -- Sizes --

    #[test]
    fn size_fr() {
        let item = parse_ok("(left=2fr|right)");
        match &item.node {
            Node::HSplit { children } => {
                let size = children[0].size.as_ref().unwrap();
                assert_eq!(size.value, 2.0);
                assert_eq!(size.unit, Unit::Fr);
            }
            other => panic!("expected HSplit, got {:?}", other),
        }
    }

    #[test]
    fn size_col() {
        let item = parse_ok("(sidebar=80col|main)");
        match &item.node {
            Node::HSplit { children } => {
                let size = children[0].size.as_ref().unwrap();
                assert_eq!(size.value, 80.0);
                assert_eq!(size.unit, Unit::Col);
            }
            other => panic!("expected HSplit, got {:?}", other),
        }
    }

    #[test]
    fn size_row() {
        let item = parse_ok("(header=5row/body)");
        match &item.node {
            Node::VSplit { children } => {
                let size = children[0].size.as_ref().unwrap();
                assert_eq!(size.value, 5.0);
                assert_eq!(size.unit, Unit::Row);
            }
            other => panic!("expected VSplit, got {:?}", other),
        }
    }

    #[test]
    fn size_px() {
        let item = parse_ok("(sidebar=200px|main)");
        match &item.node {
            Node::HSplit { children } => {
                let size = children[0].size.as_ref().unwrap();
                assert_eq!(size.value, 200.0);
                assert_eq!(size.unit, Unit::Px);
            }
            other => panic!("expected HSplit, got {:?}", other),
        }
    }

    #[test]
    fn size_percent() {
        let item = parse_ok("(left=30%|right)");
        match &item.node {
            Node::HSplit { children } => {
                let size = children[0].size.as_ref().unwrap();
                assert_eq!(size.value, 30.0);
                assert_eq!(size.unit, Unit::Percent);
            }
            other => panic!("expected HSplit, got {:?}", other),
        }
    }

    #[test]
    fn size_decimal() {
        let item = parse_ok("(left=1.5fr|right)");
        match &item.node {
            Node::HSplit { children } => {
                let size = children[0].size.as_ref().unwrap();
                assert_eq!(size.value, 1.5);
            }
            other => panic!("expected HSplit, got {:?}", other),
        }
    }

    // -- Nesting --

    #[test]
    fn nested_split() {
        let item = parse_ok("(editor|(terminal/files))");
        match &item.node {
            Node::HSplit { children } => {
                assert_eq!(children.len(), 2);
                assert!(
                    matches!(&children[1].node, Node::VSplit { children } if children.len() == 2)
                );
            }
            other => panic!("expected HSplit, got {:?}", other),
        }
    }

    #[test]
    fn nested_with_sizes() {
        let item = parse_ok("(editor=3fr|(terminal=2fr/files))");
        match &item.node {
            Node::HSplit { children } => {
                assert_eq!(children[0].size.as_ref().unwrap().value, 3.0);
                match &children[1].node {
                    Node::VSplit { children } => {
                        assert_eq!(children[0].size.as_ref().unwrap().value, 2.0);
                        assert!(children[1].size.is_none());
                    }
                    other => panic!("expected VSplit, got {:?}", other),
                }
            }
            other => panic!("expected HSplit, got {:?}", other),
        }
    }

    #[test]
    fn sized_group() {
        let item = parse_ok("((files=20%|editor1|editor2)=3fr/terminal)");
        match &item.node {
            Node::VSplit { children } => {
                assert_eq!(children[0].size.as_ref().unwrap().value, 3.0);
                assert!(
                    matches!(&children[0].node, Node::HSplit { children } if children.len() == 3)
                );
            }
            other => panic!("expected VSplit, got {:?}", other),
        }
    }

    // -- Whitespace --

    #[test]
    fn whitespace_ignored() {
        let item = parse_ok("( left | right )");
        assert!(matches!(&item.node, Node::HSplit { children } if children.len() == 2));
    }

    #[test]
    fn leading_trailing_whitespace() {
        let item = parse_ok("  (left|right)  ");
        assert!(matches!(&item.node, Node::HSplit { children } if children.len() == 2));
    }

    // -- Spec examples --

    #[test]
    fn spec_equal_horizontal() {
        parse_ok("(left|right)");
    }

    #[test]
    fn spec_equal_vertical() {
        parse_ok("(top/bottom)");
    }

    #[test]
    fn spec_nested() {
        parse_ok("(editor|(terminal/files))");
    }

    #[test]
    fn spec_unequal() {
        parse_ok("(left=2fr|right)");
    }

    #[test]
    fn spec_fixed_sidebar() {
        parse_ok("(sidebar=80col|main)");
    }

    #[test]
    fn spec_three_way_fixed() {
        parse_ok("(nav=60col|content|panel=40col)");
    }

    #[test]
    fn spec_percentage() {
        parse_ok("(left=30%|right)");
    }

    #[test]
    fn spec_nested_with_sizes() {
        parse_ok("(editor=3fr|(terminal=2fr/files))");
    }

    #[test]
    fn spec_ide_layout() {
        parse_ok("((files=20% | editor1 | editor2)=3fr / terminal)");
    }

    #[test]
    fn spec_quoted_names() {
        parse_ok("(\"Left Panel\"=2fr | \"Right Panel\")");
    }

    // -- Error cases --

    #[test]
    fn error_mixed_operators() {
        let error = parse_err("(a|b/c)");
        assert_eq!(error.kind, ErrorKind::MixedOperators);
    }

    #[test]
    fn error_unclosed_group() {
        let error = parse_err("(a|b");
        assert_eq!(error.kind, ErrorKind::UnclosedGroup);
    }

    #[test]
    fn error_unclosed_string() {
        let error = parse_err("\"hello");
        assert_eq!(error.kind, ErrorKind::UnclosedString);
    }

    #[test]
    fn error_invalid_unit() {
        let error = parse_err("(a=2xx|b)");
        assert!(matches!(error.kind, ErrorKind::InvalidUnit(_)));
    }

    #[test]
    fn error_missing_unit() {
        let error = parse_err("(a=2|b)");
        assert!(matches!(error.kind, ErrorKind::InvalidUnit(_)));
    }

    #[test]
    fn error_empty_input() {
        let error = parse_err("");
        assert_eq!(error.kind, ErrorKind::UnexpectedEof);
    }

    #[test]
    fn error_trailing_input() {
        let error = parse_err("(a|b) extra");
        assert_eq!(error.kind, ErrorKind::TrailingInput);
    }

    #[test]
    fn error_empty_group() {
        let error = parse_err("()");
        assert_eq!(error.kind, ErrorKind::EmptyGroup);
    }
}
