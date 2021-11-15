use quote::{quote, ToTokens};

use super::HtmlNode;

#[derive(Debug)]
pub enum HtmlDoctype {
  Html5,
}

impl From<HtmlDoctype> for HtmlNode {
  fn from(element: HtmlDoctype) -> Self {
    HtmlNode::Doctype(element)
  }
}

impl ToTokens for HtmlDoctype {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    quote!(ahecha::view::HtmlDoctype::Html5).to_tokens(tokens);
  }
}

impl ToString for HtmlDoctype {
  fn to_string(&self) -> String {
    "<!DOCTYPE html>".to_string()
  }
}
