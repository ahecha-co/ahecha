use quote::{quote, ToTokens};

use crate::html::{attributes::Attributes, children::Children};

use super::{HtmlComponent, HtmlNode};

#[derive(Debug)]
pub struct HtmlElement {
  pub attributes: Attributes,
  pub children: Children,
  pub name: String,
}

impl Into<HtmlNode> for HtmlElement {
  fn into(self) -> HtmlNode {
    if self.name.chars().nth(0).unwrap_or_default().is_uppercase() {
      return HtmlNode::Component(HtmlComponent {
        attributes: self.attributes,
        children: self.children,
        name: self.name,
      });
    } else {
      return HtmlNode::Element(self);
    }
  }
}

impl ToTokens for HtmlElement {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let name = &self.name;
    let attributes = &self.attributes;
    let children = &self.children;
    let name = syn::Ident::new(name, proc_macro2::Span::call_site());
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
