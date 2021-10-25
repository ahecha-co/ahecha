use crate::{renderable::Renderable, write_attributes, Attributes, Component};

use std::{
  any::TypeId,
  fmt::{Result, Write},
};

use super::node::Node;

pub struct CustomElement<'a> {
  type_id: TypeId,
  pub name: &'a str,
  pub attributes: Attributes<'a>,
  pub children: Option<Box<Node<'a>>>,
}

impl<'a> CustomElement<'a> {
  pub fn new<C: 'static>(
    name: &'a str,
    attributes: Attributes<'a>,
    children: Option<Box<Node<'a>>>,
  ) -> Self
  where
    C: Component,
  {
    Self {
      type_id: TypeId::of::<C>(),
      name,
      attributes,
      children,
    }
  }
}

impl Renderable for CustomElement<'static> {
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

impl From<CustomElement<'static>> for String {
  fn from(item: CustomElement<'static>) -> Self {
    item.to_string()
  }
}

impl From<&CustomElement<'static>> for String {
  fn from(item: &CustomElement<'static>) -> Self {
    item.to_string()
  }
}
