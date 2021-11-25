macro_rules! impl_renderable {
  ($($t:ty),*) => {
    $(impl crate::render::RenderNode for $t {
        fn render(&self) -> web_sys::Node {
          let text = gloo_utils::document().create_text_node(self.to_string().as_str());
          text.into()
        }
      }
    )*
  };
}

impl_renderable!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);

#[cfg(test)]
mod test {
  use ahecha_tuple_list::tuple_list;

  use crate::{render::RenderString, HtmlElement};

  #[test]
  fn test_render_into() {
    let element = HtmlElement {
      name: "div",
      attributes: (),
      children: Some(tuple_list!(
        HtmlElement {
          name: "span",
          attributes: (),
          children: Some(tuple_list!("Hello", " ", 1u8)),
        },
        ", ",
        "World",
        " ",
        2u8,
      )),
    };

    assert_eq!(element.render(), "<div><span>Hello 1</span>, World 2</div>");
  }
}
