use quote::{quote, ToTokens};

use super::node::HtmlNode;

#[derive(Debug, Default)]
pub struct Children {
  pub nodes: Vec<HtmlNode>,
}

impl ToTokens for Children {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let mut list = quote! { () };

    for node in self.nodes.iter().rev() {
      list = quote! { (#node, #list) }
    }

    quote! { Some(#list) }.to_tokens(tokens);
  }
}
