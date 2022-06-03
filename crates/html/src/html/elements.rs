use super::children::Children;
use crate::{AttributeValue, Attributes};

#[derive(Debug, Clone)]
pub struct Element {
  pub attributes: Attributes,
  pub children: Children,
  pub name: &'static str,
}

impl Element {
  pub fn replace(&mut self, el: Element) {
    self.attributes = el.attributes;
    self.children = el.children;
    self.name = el.name;
  }

  pub fn attr(&self, name: &str) -> Option<AttributeValue> {
    self
      .attributes
      .attrs
      .iter()
      .find(|(key, _)| key.as_str() == name)
      .map(|(_, value)| value.clone())
  }

  pub fn has_attr(&self, name: &str) -> bool {
    self.attr(name).is_some()
  }

  pub fn has_attr_value(&self, name: &str, value: &str) -> bool {
    self
      .attr(name)
      .map(|v| v.to_string() == value)
      .unwrap_or(false)
  }

  pub fn is_slot(&self) -> bool {
    self.name == "slot"
  }
}
