use quote::{quote, ToTokens};

use super::HtmlNode;

#[derive(Debug)]
pub enum HtmlDoctype {
  Html5,
}

impl Into<HtmlNode> for HtmlDoctype {
  fn into(self) -> HtmlNode {
    return HtmlNode::Doctype(self);
  }
}

impl ToTokens for HtmlDoctype {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    quote!(ita::view::HtmlDoctype::Html5).to_tokens(tokens);
  }
}

impl ToString for HtmlDoctype {
  fn to_string(&self) -> String {
    "<!DOCTYPE html>".to_string()
  }
}
