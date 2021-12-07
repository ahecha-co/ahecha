use crate::html::{Doctype, Element};

pub enum Node {
  CustomElement,
  Document(Doctype, Vec<Node>),
  Element(Element),
  Fragment(Vec<Node>),
  Text(String),
}

impl From<Vec<Node>> for Node {
  fn from(item: Vec<Node>) -> Node {
    Node::Fragment(item)
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

impl_renderable!(&str, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
