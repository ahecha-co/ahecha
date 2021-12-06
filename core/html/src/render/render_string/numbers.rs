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
  };

  #[test]
  fn test_render_into() {
    let element = Element {
      name: "div",
      attributes: vec![],
      children: vec![
        Node::Element(Element {
          name: "span",
          attributes: vec![],
          children: vec![
            Node::Text("Hello".to_owned()),
            Node::Text(" ".to_owned()),
            Node::Text("1".to_owned()),
          ],
        }),
        Node::Text(", World 2".to_owned()),
      ],
    };

    assert_eq!(element.render(), "<div><span>Hello 1</span>, World 2</div>");
  }
}
