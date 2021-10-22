use etagere_codegen::*;
use etagere_view::ToHtml;

mod etagere {
  pub use etagere_view as view;
}

#[component]
struct MyCustomElement {}

#[test]
fn simple_html_test() {
  let res = html! { <div></div> }.to_html();
  assert_eq!(res, "<div/>");
}

#[test]
fn simple_html_with_text_test() {
  let res = html! { <div>"Text"</div> }.to_html();
  assert_eq!(res, "<div>Text</div>");
}

#[test]
fn simple_html_with_attributes_test() {
  let res = html! { <div class="some_class">"Text"</div> }.to_html();
  assert_eq!(res, "<div class=\"some_class\">Text</div>");
}

#[test]
fn custom_element_test() {
  let res = html! { <MyCustomElement>"Text"</MyCustomElement> }.to_html();
  assert_eq!(res, "<my-custom-element>Text</my-custom-element>");
}
