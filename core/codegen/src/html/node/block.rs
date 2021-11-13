use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(Debug)]
pub struct HtmlBlock {
  pub block: String,
}

impl ToTokens for HtmlBlock {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let block: TokenStream = self.block.parse().unwrap();
    tokens.extend(quote! {
      #block
    });
  }
}

impl ToString for HtmlBlock {
  fn to_string(&self) -> String {
    self.block.clone()
  }
}
