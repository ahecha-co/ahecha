use std::borrow::Cow;

use etagere_codegen::*;
use etagere_view::{CustomElement, Html};

mod etagere {
  pub use etagere_view as view;
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

#[derive(Default)]
struct MyCustomElement<'a> {
  attributes: Vec<(&'a str, Cow<'a, str>)>,
  children: Html<'a>,
}

impl<'a> CustomElement<'a> for MyCustomElement<'a> {
  fn create(&mut self, attributes: Vec<(&'a str, Cow<'a, str>)>, children: Html<'a>) {
    self.attributes = attributes;
    self.children = children;
  }

  fn attributes(&self) -> Vec<(&'a str, Cow<'a, str>)> {
    self.attributes.clone()
  }

  fn render(&self) -> Html<'a> {
    html! {
      <strong>"Custom Element"</strong>
    }
  }
}

#[test]
fn custom_element_test() {
  let res: String = html! { <MyCustomElement>"Text"</MyCustomElement> }.into();
  assert_eq!(
    res,
    "<my-custom-element><strong>Custom Element</strong></my-custom-element>"
  );
}

#[derive(Default)]
struct PostElement<'a> {
  attributes: Vec<(&'a str, Cow<'a, str>)>,
  children: Html<'a>,
}

impl<'a> CustomElement<'a> for PostElement<'a> {
  fn create(&mut self, attributes: Vec<(&'a str, Cow<'a, str>)>, children: Html<'a>) {
    self.attributes = attributes;
    self.children = children;
  }

  fn attributes(&self) -> Vec<(&'a str, Cow<'a, str>)> {
    self.attributes.clone()
  }

  fn render(&self) -> Html<'a> {
    let title = self
      .attributes()
      .iter()
      .find(|(key, _)| *key == "title")
      .unwrap()
      .1
      .clone();
    let body = self
      .attributes()
      .iter()
      .find(|(key, _)| *key == "body")
      .unwrap()
      .1
      .clone();
    html! {
      <div>
        <h1>{ title }</h1>
        <p>{ body }</p>
      </div>
    }
  }
}

#[test]
fn custom_element_with_props_test() {
  let res: String = html! { <PostElement title="Hello" body="World">"Text"</PostElement> }.into();
  assert_eq!(
    res,
    "<post-element title=\"Hello\" body=\"World\"><h1>Hello</h1><p>World</p></post-element>"
  );
}

#[test]
fn html_with_code_block_test() {
  let text = "Text";
  let res: String = html! { <div>{ text }</div> }.into();
  assert_eq!(res, "<div>Text</div>");
}
