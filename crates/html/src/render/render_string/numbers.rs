use std::fmt::{Result, Write};

macro_rules! impl_renderable {
  ($($t:ty),*) => {
    $(
      impl crate::render::RenderString for $t {
        fn render_into<W: Write>(self, writer: &mut W) -> Result {
          write!(writer, "{}", self)
        }
      }

      impl crate::render::RenderString for & $t {
        fn render_into<W: Write>(self, writer: &mut W) -> Result {
          write!(writer, "{}", self)
        }
      }
    )*
  };
}

impl_renderable!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

#[cfg(test)]
mod test {
  use crate::{
    html::{Element, Node},
    render::RenderString,
    Children, NodeId,
  };

  #[test]
  fn test_render_into() {
    let element = Element {
      name: "div",
      attributes: Default::default(),
      children: Children::default()
        .set(Node::Element(
          Element {
            name: "span",
            attributes: Default::default(),
            children: Children::default()
              .set(Node::Text("Hello".to_owned()))
              .set(Node::Text(" ".to_owned()))
              .set(Node::Text("1".to_owned())),
          },
          NodeId::new(),
        ))
        .set(Node::Text(", World 2".to_owned())),
    };

    assert_eq!(element.render(), "<div><span>Hello 1</span>, World 2</div>");
  }
}
