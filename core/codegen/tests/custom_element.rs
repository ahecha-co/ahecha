use ahecha::view::RenderString;
use ahecha_codegen::*;
// use wasm_bindgen::prelude::*;

mod ahecha {
  pub use ahecha_view as view;
}

#[test]
fn test_custom_element_without_macro_attr() {
  struct TestCustomElement {}

  impl TestCustomElement {
    fn view(&self) -> String {
      html!(
        <div class="main">I am a custom element</div>
      )
      .render()
    }
  }

  impl RenderString for TestCustomElement {
    fn render_into<W: std::fmt::Write>(self, writer: &mut W) -> std::fmt::Result {
      write!(writer, "{}", self.view())
    }
  }

  let res = html! {
    <TestCustomElement></TestCustomElement>
  };

  assert_eq!(
    res.render(),
    "<test-custom-element><div class=\"main\">I am a custom element</div></test-custom-element>"
  );
}

#[test]
fn test_custom_element_with_macro_attr() {
  #[custom_element]
  fn FnCustomElement() {
    html!(
      <div class="main">I am a custom element</div>
    )
  }

  let res = html! {
    <FnCustomElement></FnCustomElement>
  };

  assert_eq!(
    res.render(),
    "<fn-custom-element><div class=\"main\">I am a custom element</div></fn-custom-element>"
  );
}
