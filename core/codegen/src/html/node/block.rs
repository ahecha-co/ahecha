use quote::{quote, ToTokens};

#[derive(Debug)]
pub struct HtmlBlock {
  pub block: String,
}

impl ToTokens for HtmlBlock {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let block = &self.block;
    tokens.extend(quote! {
      #block
    });
  }
}
