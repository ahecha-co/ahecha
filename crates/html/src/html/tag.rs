use crate::ToHtmlString;

use super::{attribute::AttributeValue, node::Node};

pub struct Tag {
  name: String,
  attributes: Vec<(String, AttributeValue)>,
  children: Vec<Node>,
}

impl Tag {
  pub fn new(name: &str) -> Self {
    Self {
      name: name.to_owned(),
      attributes: vec![],
      children: vec![],
    }
  }

  pub fn set_atttribute(mut self, attribute: &str, value: impl Into<AttributeValue>) -> Self {
    if let Some(index) = self
      .attributes
      .iter()
      .position(|(attr, _)| attr == attribute)
    {
      self
        .attributes
        .insert(index, (attribute.to_owned(), value.into()));
    } else {
      self.attributes.push((attribute.to_owned(), value.into()));
    }

    self
  }

  pub fn add_child(mut self, child: Node) -> Self {
    self.children.push(child);
    self
  }
}

impl ToHtmlString for Tag {
  fn render_into<W: std::fmt::Write>(self, buffer: &mut W) -> std::fmt::Result {
    write!(buffer, "<{}", &self.name)?;

    if !self.attributes.is_empty() {
      write!(buffer, " ")?;
      self.attributes.render_into(buffer)?;
    }

    if self.children.is_empty() {
      let self_closing_tags = [
        "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param",
        "source", "track", "wbr",
      ];

      if self_closing_tags.contains(&self.name.as_str()) {
        write!(buffer, "/>")
      } else {
        write!(buffer, "></{}>", &self.name)
      }
    } else {
      write!(buffer, ">")?;
      self.children.render_into(buffer)?;
      write!(buffer, "</{}>", &self.name)
    }
  }
}
