use quote::{quote, ToTokens};

use crate::html::{attributes::Attributes, children::Children};

use super::{HtmlCustomElement, HtmlNode};

#[derive(Debug)]
pub struct HtmlElement {
  pub attributes: Attributes,
  pub children: Children,
  pub name: String,
}

impl Into<HtmlNode> for HtmlElement {
  fn into(self) -> HtmlNode {
    if self.name.chars().nth(0).unwrap_or_default().is_uppercase() {
      HtmlNode::CustomElement(HtmlCustomElement {
        attributes: self.attributes,
        children: self.children,
        name: self.name,
      })
    } else {
      HtmlNode::Element(self)
    }
  }
}

impl ToTokens for HtmlElement {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let attributes = &self.attributes;
    let children = &self.children;
    let name = syn::Ident::new(&self.name, proc_macro2::Span::call_site());

    let element = quote!(
      ita::view::HtmlElement {
        attributes: #attributes,
        children: #children,
        name: stringify!(#name),
      }
    );
    element.to_tokens(tokens);
  }
}

impl ToString for HtmlElement {
  fn to_string(&self) -> String {
    format!("<{}>...</{}>", self.name, self.name)
  }
}
