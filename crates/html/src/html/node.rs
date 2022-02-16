use crate::html::{Doctype, Element};

pub enum Node {
  Document(Doctype, Vec<Node>),
  Element(Element),
  Fragment(Vec<Node>),
  None,
  Text(String),
}

impl From<Vec<Node>> for Node {
  fn from(item: Vec<Node>) -> Node {
    Node::Fragment(item)
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
      Some(node) => Node::Fragment(node),
      None => Node::None,
    }
  }
}

impl ToString for Node {
  fn to_string(&self) -> String {
    match self {
      Node::Document(doctype, nodes) => {
        format!(
          "{}{}",
          doctype.to_string(),
          nodes
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join("\n")
        )
      }
      Node::Element(el) => el.to_string(),
      Node::Fragment(nodes) => nodes
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
