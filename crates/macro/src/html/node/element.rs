use quote::{quote, ToTokens};

use super::Node;
use crate::html::{attributes::Attributes, children::Children, tag_name::TagName};

#[derive(Debug)]
pub struct HtmlElement {
  pub attributes: Attributes,
  pub children: Children,
  pub name: TagName,
}

impl From<HtmlElement> for Node {
  fn from(element: HtmlElement) -> Self {
    Node::Element(element)
  }
}

impl ToTokens for HtmlElement {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let attributes = &self.attributes;
    let children = &self.children;
    let name = &self.name.to_string();

    let element = quote!(
      ahecha::tag(#name)
        #children
        #attributes
    );
    element.to_tokens(tokens);
  }
}

impl ToString for HtmlElement {
  fn to_string(&self) -> String {
    format!("<{}>...</{}>", self.name, self.name)
  }
}
