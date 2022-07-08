use std::fmt::Debug;

use quote::{quote, ToTokens};
use syn::{parse::Parse, Block};

pub struct AttributeBlock {
  pub block: Block,
}

impl Debug for AttributeBlock {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Attribute {{ block: {{}} }}",)
  }
}

impl Parse for AttributeBlock {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    Ok(AttributeBlock {
      block: input.parse()?,
    })
  }
}

impl ToTokens for AttributeBlock {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let block = &self.block;
    quote! { .dyn_c( || #block ) }.to_tokens(tokens);
  }
}
