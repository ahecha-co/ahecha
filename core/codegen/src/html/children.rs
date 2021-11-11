use std::ops::Not;

use quote::{quote, ToTokens};

use super::node::HtmlNode;

#[derive(Debug, Default)]
pub struct Children {
  pub nodes: Vec<HtmlNode>,
}

impl ToTokens for Children {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let list = self.nodes.iter().map(|node| quote!(#node));
    quote!(Some((#(#list),*))).to_tokens(tokens);
  }
}
