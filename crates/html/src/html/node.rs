use crate::ToHtmlString;

use super::tag::Tag;

pub enum Node {
  Fragment(Vec<Node>),
  Tag(Tag),
  Text(String),
}

impl ToHtmlString for Node {
  fn render_into<W: std::fmt::Write>(self, buffer: &mut W) -> std::fmt::Result {
    match self {
      Self::Fragment(value) => value.render_into(buffer)?,
      Self::Tag(value) => value.render_into(buffer)?,
      Self::Text(value) => write!(buffer, "{}", value)?,
    }

    Ok(())
  }
}

impl From<Tag> for Node {
  fn from(item: Tag) -> Self {
    Self::Tag(item)
  }
}

impl From<String> for Node {
  fn from(item: String) -> Self {
    Self::Text(item)
  }
}

impl From<&str> for Node {
  fn from(item: &str) -> Self {
    Self::Text(item.to_owned())
  }
}

impl From<Vec<Node>> for Node {
  fn from(item: Vec<Node>) -> Self {
    Node::Fragment(item)
  }
}

macro_rules! impl_node_from_t {
  ($($t:ty),*) => {
    $(
      impl From<$t> for Node {
        fn from(item: $t) -> Self {
          Self::Text(item.to_string())
        }
      }
    )*
  };
}

impl_node_from_t!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, bool, f32, f64);
