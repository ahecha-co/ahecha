use std::fmt::Debug;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::Parse;

pub struct HtmlBlock {
  pub block: TokenStream,
}

impl ToTokens for HtmlBlock {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let stringified = self.block.to_string();
    let block = if stringified.contains(";") {
      self.block.clone()
    } else {
      stringified
        .replace("{", "")
        .replace("}", "")
        .parse()
        .unwrap()
    };

    if stringified.contains("html!") || stringified.contains("ahecha::") {
      dbg!(">>>>>>>>>>> new_dyn", &block.to_string());
      quote! (
        ahecha::sycamore::view::View::new_dyn(cx, move || {
          ahecha::sycamore::view::IntoView::create( & #block )
        })
      )
    } else {
      dbg!(">>>>>>>>>>> IntoView", &block.to_string());
      quote! (ahecha::sycamore::view::IntoView::create( & #block ) )
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
    match input.parse::<syn::Block>() {
      Ok(block) => Ok(HtmlBlock {
        block: quote!(#block),
      }),
      Err(err) => Err(err),
    }
  }
}
