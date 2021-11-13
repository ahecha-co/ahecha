use ahecha::view::Render;
use ahecha_codegen::*;

mod ahecha {
  pub use ahecha_view as view;
}

#[test]
fn test_custom_element_without_macro_attr() {
  struct CustomElement {}

  impl CustomElement {
    fn view(&self) -> String {
      html!(
        <div class="main">I am a custom element</div>
      )
      .render()
    }
  }

  impl Render for CustomElement {
    fn render_into<W: std::fmt::Write>(self, writer: &mut W) -> std::fmt::Result {
      write!(writer, "{}", self.view())
    }
  }

  let res = html! {
    <CustomElement></CustomElement>
  };

  assert_eq!(
    res.render(),
    "<custom-element><div class=\"main\">I am a custom element</div></custom-element>"
  );
}

#[test]
fn test_custom_element_with_macro_attr() {
  #[custom_element]
  fn CustomElement() {
    html!(
      <div class="main">I am a custom element</div>
    )
  }

  let res = html! {
    <CustomElement></CustomElement>
  };

  assert_eq!(
    res.render(),
    "<custom-element><div class=\"main\">I am a custom element</div></custom-element>"
  );
}
