use quote::{quote, ToTokens};

use crate::html::{
  attributes::{attribute::key_value::AttributeValue, Attributes},
  children::Children,
};

#[derive(Debug)]
pub struct LiveView {
  pub attributes: Attributes,
  pub children: Children,
  pub id: AttributeValue,
}

impl ToTokens for LiveView {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let id = &self.id;
    let attributes = &self.attributes;
    let children = &self.children;

    let element = quote!(
      ahecha::html::Node::LiveView(ahecha::html::LiveView {
        attributes: #attributes,
        children: #children,
        id: #id .to_string(),
      })
    );
    element.to_tokens(tokens);
  }
}

impl ToString for LiveView {
  fn to_string(&self) -> String {
    "<live-view >...</live-view>".to_owned()
  }
}
