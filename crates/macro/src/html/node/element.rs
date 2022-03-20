use quote::{quote, ToTokens};

use super::{HtmlCustomElement, Node};
use crate::html::{attributes::Attributes, children::Children, tag_name::TagName};

#[derive(Debug)]
pub struct HtmlElement {
  pub attributes: Attributes,
  pub children: Children,
  pub name: TagName,
}

impl From<HtmlElement> for Node {
  fn from(element: HtmlElement) -> Self {
    if element
      .name
      .to_string()
      .chars()
      .next()
      .unwrap_or_default()
      .is_uppercase()
    {
      // if element.name.to_string().ends_with("Partial") {
      //   Node::LiveView(LiveView {
      //     attributes: element.attributes,
      //     children: element.children,
      //     name: element.name,
      //   })
      // } else {
      Node::CustomElement(HtmlCustomElement {
        attributes: element.attributes,
        children: element.children,
        name: element.name,
      })
      // }
    } else {
      Node::Element(element)
    }
  }
}

impl ToTokens for HtmlElement {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let attributes = &self.attributes;
    let children = &self.children;
    let name = &self.name.to_string();

    let element = quote!(
      ahecha::html::Node::Element(ahecha::html::Element {
        attributes: #attributes,
        children: #children,
        name: #name,
      })
    );
    element.to_tokens(tokens);
  }
}

impl ToString for HtmlElement {
  fn to_string(&self) -> String {
    format!("<{}>...</{}>", self.name, self.name)
  }
}
