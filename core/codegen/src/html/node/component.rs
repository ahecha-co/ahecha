use quote::{quote, ToTokens};

use crate::html::{attributes::Attributes, children::Children};

#[derive(Debug)]
pub struct HtmlComponent {
  pub attributes: Attributes,
  pub children: Children,
  pub name: String,
}

impl ToTokens for HtmlComponent {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let name = &self.name;
    let attributes = &self.attributes;
    let children = &self.children;
    let element = quote!(
      ita::view::HtmlElement {
        name: stringify!(#name),
        attributes: #attributes,
        children: #children,
      }
    );
    element.to_tokens(tokens);
  }
}
