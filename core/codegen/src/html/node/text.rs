use quote::{quote, ToTokens};

#[derive(Debug)]
pub struct HtmlText {
  pub text: String,
}

impl ToTokens for HtmlText {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let text = &self.text;
    tokens.extend(quote! {
      #text
    });
  }
}
