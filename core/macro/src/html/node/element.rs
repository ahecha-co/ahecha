use quote::{quote, ToTokens};

use crate::html::{attributes::Attributes, children::Children};

use super::{HtmlCustomElement, HtmlNode, HtmlPartial};

#[derive(Debug)]
pub struct HtmlElement {
  pub attributes: Attributes,
  pub children: Children,
  pub name: syn::Ident,
}

impl From<HtmlElement> for HtmlNode {
  fn from(element: HtmlElement) -> Self {
    if element
      .name
      .to_string()
      .chars()
      .next()
      .unwrap_or_default()
      .is_uppercase()
    {
      if element.name.to_string().ends_with("Partial") {
        HtmlNode::Partial(HtmlPartial {
          attributes: element.attributes,
          children: element.children,
          name: element.name,
        })
      } else {
        HtmlNode::CustomElement(HtmlCustomElement {
          attributes: element.attributes,
          children: element.children,
          name: element.name,
        })
      }
    } else {
      HtmlNode::Element(element)
    }
  }
}

impl ToTokens for HtmlElement {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let attributes = &self.attributes;
    let children = &self.children;
    let name = &self.name;

    let element = quote!(
      ahecha::html::HtmlElement {
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
