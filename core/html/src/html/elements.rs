use super::node::Node;

pub enum AttributeValue {
  String(String),
  Bool(bool),
}

pub struct Element {
  pub attributes: Vec<(String, AttributeValue)>,
  pub children: Vec<Node>,
  pub name: &'static str,
}
