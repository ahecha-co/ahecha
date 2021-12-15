use quote::{quote, ToTokens};

use crate::html::{
  attributes::{Attribute, AttributeValue, Attributes},
  children::Children,
};

#[derive(Debug)]
pub struct HtmlPartial {
  pub attributes: Attributes,
  pub children: Children,
  pub name: syn::Ident,
}

impl ToTokens for HtmlPartial {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let ident = &self.name;
    let attributes = &self.attributes;
    let children = &self.children;

    let mut attrs = vec![];

    for Attribute { key, value, .. } in attributes.attrs.iter() {
      match value {
        AttributeValue::Block(block) => attrs.push(quote!( #key: #block )),
        AttributeValue::Lit(lit) => attrs.push(quote!( #key: #lit )),
      }
    }

    if !children.nodes.is_empty() {
      attrs.push(quote!( #children ))
    }

    let params = if attrs.is_empty() {
      quote!()
    } else {
      quote!(#ident ::ViewParams { #(#attrs,)* })
    };

    let element = quote!(#ident ::view( #params ));

    element.to_tokens(tokens);
  }
}

impl ToString for HtmlPartial {
  fn to_string(&self) -> String {
    format!("<{}>...</{}>", self.name, self.name)
  }
}
