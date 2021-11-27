#[cfg(feature = "backend")]
mod backend {
  use ahecha::view::RenderString;
  use ahecha_macro::*;
  mod ahecha {
    pub use ahecha_html as view;
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
}
