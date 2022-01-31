#[cfg(feature = "backend")]
mod backend {
  use ahecha::prelude::*;

  #[test]
  fn test_custom_element_with_macro_attr() {
    #[custom_element]
    fn FnCustomElement() -> ahecha::html::Node {
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
}