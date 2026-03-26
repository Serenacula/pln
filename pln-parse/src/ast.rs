use serde::Serialize;

/// A byte-offset span in the source text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

/// A layout item: a node with an optional size annotation.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Item {
    #[serde(flatten)]
    pub node: Node,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<Size>,
    pub span: Span,
}

/// A layout node: either a leaf panel or a split group.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Node {
    Panel { name: String },
    HSplit { children: Vec<Item> },
    VSplit { children: Vec<Item> },
}

/// A size annotation.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Size {
    pub value: f64,
    pub unit: Unit,
}

/// Size units.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum Unit {
    #[serde(rename = "fr")]
    Fr,
    #[serde(rename = "col")]
    Col,
    #[serde(rename = "row")]
    Row,
    #[serde(rename = "px")]
    Px,
    #[serde(rename = "%")]
    Percent,
}
