use crate::{renderable::Renderable, write_attributes, Attributes};

use std::fmt::{Result, Write};

use super::node::Node;

pub struct Tag<'a> {
  pub name: &'a str,
  pub attributes: Attributes<'a>,
  pub children: Option<Box<Node<'a>>>,
}

impl Renderable for Tag<'static> {
  fn writer<W: Write>(&self, writer: &mut W) -> Result {
    match &self.children {
      None => {
        write!(writer, "<{}", self.name)?;
        write_attributes(self.attributes.clone(), writer)?;
        write!(writer, "/>")
      }
      Some(children) => {
        write!(writer, "<{}", self.name)?;
        write_attributes(self.attributes.clone(), writer)?;
        write!(writer, ">")?;
        children.writer(writer)?;
        write!(writer, "</{}>", self.name)
      }
    }
  }
}

impl From<Tag<'static>> for String {
  fn from(item: Tag<'static>) -> Self {
    item.to_string()
  }
}

impl From<&Tag<'static>> for String {
  fn from(item: &Tag<'static>) -> Self {
    item.to_string()
  }
}

impl From<Box<Tag<'static>>> for String {
  fn from(item: Box<Tag<'static>>) -> Self {
    item.to_string()
  }
}
