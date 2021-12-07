use super::node::Node;

pub enum AttributeValue {
  Bool(bool),
  None,
  String(String),
}

pub struct Element {
  pub attributes: Vec<(String, AttributeValue)>,
  pub children: Vec<Node>,
  pub name: &'static str,
}
