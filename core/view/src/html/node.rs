use crate::renderable::Renderable;

use super::{custom_element::CustomElement, tag::Tag};

#[derive(Clone)]
pub enum Node<'a> {
  CustomElement(CustomElement<'a>),
  Empty,
  List(Vec<Node<'a>>),
  Tag(Tag<'a>),
  Text(String),
}

impl Renderable for Node<'static> {
  fn writer<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
    use Node::*;

    match self {
      CustomElement(el) => el.writer(writer),
      Empty => Ok(()),
      List(list) => list.writer(writer),
      Tag(tag) => tag.writer(writer),
      Text(text) => write!(writer, "{}", text),
    }
  }
}

impl<'a> Renderable for Vec<Node<'static>> {
  fn writer<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
    self.iter().try_for_each(|n| n.writer(writer))
  }
}

impl<'a> From<Node<'static>> for String {
  fn from(item: Node<'static>) -> Self {
    item.to_string()
  }
}
