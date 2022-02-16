use serde::{Deserialize, Serialize};

use super::node::Node;
use std::fmt::Write;

#[derive(Serialize, Deserialize)]
pub struct SerializableAttributeValue(pub Option<String>);

impl ToString for SerializableAttributeValue {
  fn to_string(&self) -> String {
    match &self.0 {
      Some(value) => value.clone(),
      None => String::new(),
    }
  }
}

pub enum AttributeValue {
  Bool(bool),
  None,
  String(String),
}

impl ToString for AttributeValue {
  fn to_string(&self) -> String {
    match self {
      AttributeValue::Bool(value) => value.to_string(),
      AttributeValue::None => "".to_owned(),
      AttributeValue::String(text) => text.clone(),
    }
  }
}

pub struct Element {
  pub attributes: Vec<(String, SerializableAttributeValue)>,
  pub children: Vec<Node>,
  pub name: &'static str,
}

impl ToString for Element {
  fn to_string(&self) -> String {
    let mut buffer = String::new();

    write!(&mut buffer, "<{}", self.name).unwrap();

    if !self.attributes.is_empty() {
      let attr = self
        .attributes
        .iter()
        .map(|(key, value)| {
          let value = value.to_string();

          if value.is_empty() {
            format!("{}", key)
          } else {
            format!("{}={}", key, value)
          }
        })
        .collect::<Vec<_>>()
        .join(" ");
      write!(&mut buffer, " {}", attr).unwrap();
    }

    if self.children.is_empty() {
      write!(&mut buffer, "/>").unwrap();
    } else {
      let children = self
        .children
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join("\n");
      write!(&mut buffer, ">{}</{}>", children, self.name).unwrap();
    }

    buffer
  }
}
