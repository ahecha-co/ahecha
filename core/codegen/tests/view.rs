use std::borrow::Cow;

use etagere_codegen::*;
use etagere_view::{Attributes, CustomElement, Html};

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

#[test]
fn custom_element_test() {
  #[derive(Default)]
  struct MyCustomElement<'a> {
    children: Html<'a>,
  }

  impl<'a> CustomElement<'a> for MyCustomElement<'a> {
    type Attributes = ();

    fn create(&mut self, _attributes: Self::Attributes, children: Html<'a>) {
      self.children = children;
    }

    fn render(&self) -> Html<'a> {
      html! {
        <strong>"Custom Element"</strong>
      }
    }
  }

  let res: String = html! { <MyCustomElement>"Text"</MyCustomElement> }.into();
  assert_eq!(
    res,
    "<my-custom-element><strong>Custom Element</strong></my-custom-element>"
  );
}

#[test]
fn custom_element_with_props_test() {
  #[derive(Default, Clone)]
  struct Post {
    title: String,
    body: String,
  }
  #[derive(Default)]
  struct PostElement<'a> {
    attributes: Post,
    children: Html<'a>,
  }

  impl<'a> CustomElement<'a> for PostElement<'a> {
    type Attributes = Post;

    fn create(&mut self, attributes: Self::Attributes, children: Html<'a>) {
      self.attributes = attributes;
      self.children = children;
    }

    fn attributes(&self) -> Self::Attributes {
      self.attributes.clone()
    }

    fn render(&self) -> Html<'a> {
      html! {
        <div>
          <h1>{ self.attributes.title }</h1>
          <p>{ self.attributes.body }</p>
        </div>
      }
    }
  }

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
