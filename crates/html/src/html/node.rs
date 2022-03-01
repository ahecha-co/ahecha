use crate::{
  html::{Doctype, Element},
  Children,
};

pub enum Node {
  Comment(Children),
  CustomElement,
  Document(Doctype, Children),
  Element(Element),
  Fragment(Children),
  None,
  Text(String),
}

impl From<Vec<Node>> for Node {
  fn from(children: Vec<Node>) -> Node {
    Node::Fragment(Children { children })
  }
}

impl From<Option<Node>> for Node {
  fn from(item: Option<Node>) -> Node {
    match item {
      Some(node) => node,
      None => Node::None,
    }
  }
}

impl From<Option<Vec<Node>>> for Node {
  fn from(item: Option<Vec<Node>>) -> Node {
    match item {
      Some(children) => Node::Fragment(Children { children }),
      None => Node::None,
    }
  }
}

impl ToString for Node {
  fn to_string(&self) -> String {
    match self {
      Node::Comment(children) => children
        .children
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join("\n"),
      Node::CustomElement => todo!("CustomElement"),
      Node::Document(doctype, children) => {
        format!(
          "{}{}",
          doctype.to_string(),
          children
            .children
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join("\n")
        )
      }
      Node::Element(el) => el.to_string(),
      Node::Fragment(children) => children
        .children
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<_>>()
        .join("\n"),
      Node::None => String::new(),
      Node::Text(text) => text.clone(),
    }
  }
}

macro_rules! impl_renderable {
  ($($t:ty),*) => {
    $(
      impl From<$t> for Node {
        fn from(item: $t) -> Node {
          Node::Text(item.to_string())
        }
      }

      impl From<& $t> for Node {
        fn from(item: & $t) -> Node {
          Node::Text(item.to_string())
        }
      }
    )*
  };
}

impl_renderable!(
  String, &str, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);
