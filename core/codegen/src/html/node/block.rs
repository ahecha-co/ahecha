use std::fmt::Debug;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::Parse;

pub struct HtmlBlock {
  pub block: TokenStream,
}

impl ToTokens for HtmlBlock {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let block = self.block.clone();
    quote! {
      #block
    }
    .to_tokens(tokens);
  }
}

impl Debug for HtmlBlock {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let block = self.block.clone();
    write!(f, "HtmlBlock {{ block: {:?} }}", quote!( stringify(#block)))
  }
}

impl ToString for HtmlBlock {
  fn to_string(&self) -> String {
    self.block.to_string()
  }
}

impl Parse for HtmlBlock {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let block = input.parse::<syn::Block>()?;
    Ok(HtmlBlock {
      block: quote!(#block),
    })
  }
}
