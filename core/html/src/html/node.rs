use crate::html::{Doctype, Element};

pub enum Node {
  CustomElement,
  Document(Doctype, Vec<Node>),
  Element(Element),
  Fragment(Vec<Node>),
  Text(String),
}
