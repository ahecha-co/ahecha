use std::fmt::{Result, Write};

macro_rules! impl_renderable {
  ($($t:ty),*) => {
    $(impl crate::backend::render::Render for $t {
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
  use crate::backend::elements::{tag::TagElement, HtmlElement, Render};

  #[test]
  fn test_render_into() {
    let element = TagElement::new(
      "div",
      (),
      (
        TagElement::new("span", (), ("Hello", " ", 1u8).into()),
        ", ",
        "World",
        " ",
        2u8,
      )
        .into(),
    );

    assert_eq!(
      element.to_string(),
      "<div><span>Hello 1</span>, World 2</div>"
    );
  }
}
