use etagere_codegen::*;

mod etagere {
  pub use etagere_view as view;
}

#[component]
struct MyCustomElement {}

#[test]
fn html_tag_test() {
  let res: String = html! { <div></div> };
  assert_eq!(res, "<div/>");
}

#[test]
fn html_tag_with_text_test() {
  let res: String = html! { <div>"Text"</div> };
  assert_eq!(res, "<div>Text</div>");
}

#[test]
fn html_tag_with_attributes_test() {
  let res: String = html! { <div class="some_class">"Text"</div> };
  assert_eq!(res, "<div class=\"some_class\">Text</div>");
}

#[test]
fn custom_element_test() {
  let res: String = html! { <MyCustomElement>"Text"</MyCustomElement> };
  assert_eq!(res, "<my-custom-element>Text</my-custom-element>");
}
