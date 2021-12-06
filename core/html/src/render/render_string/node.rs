use crate::{html::Node, RenderString};

impl RenderString for Node {
  fn render_into<W: std::fmt::Write>(self, writer: &mut W) -> std::fmt::Result {
    match self {
      Self::CustomElement => unimplemented!(),
      Self::Document(doctype, elements) => {
        doctype.render_into(writer)?;
        elements.render_into(writer)?;
      }
      Self::Element(element) => element.render_into(writer)?,
      Self::Fragment(elements) => elements.render_into(writer)?,
      Self::Text(text) => text.render_into(writer)?,
    }

    Ok(())
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

impl_renderable!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

impl From<&str> for Node {
  fn from(item: &str) -> Node {
    Node::Text(item.to_owned())
  }
}
