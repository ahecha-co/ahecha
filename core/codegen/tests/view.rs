use etagere_codegen::*;
use etagere_view::{CustomElement, Html};

mod etagere {
  pub use etagere_view as view;
}

#[derive(Default)]
struct MyCustomElement<'a> {
  attributes: Vec<(&'a str, &'a str)>,
  children: Html<'a>,
}

impl<'a> CustomElement<'a> for MyCustomElement<'a> {
  fn create(&mut self, attributes: Vec<(&'a str, &'a str)>, children: Html<'a>) {
    self.attributes = attributes;
    self.children = children;
  }

  fn attributes(&self) -> Vec<(&'a str, &'a str)> {
    self.attributes.clone()
  }

  fn render(&self) -> Html<'a> {
    html! {
      <strong>"Custom Element"</strong>
    }
  }
}

#[test]
fn html_tag_test() {
  let res: String = html! { <div></div> }.into();
  assert_eq!(res, "<div/>");
}

#[test]
fn html_tag_with_text_test() {
  let res: String = html! { <div>"Text"</div> }.into();
  assert_eq!(res, "<div>Text</div>");
}

#[test]
fn html_tag_with_attributes_test() {
  let res: String = html! { <div class="some_class">"Text"</div> }.into();
  assert_eq!(res, "<div class=\"some_class\">Text</div>");
}

#[test]
fn custom_element_test() {
  let res: String = html! { <MyCustomElement>"Text"</MyCustomElement> }.into();
  assert_eq!(
    res,
    "<my-custom-element><strong>Custom Element</strong></my-custom-element>"
  );
}
