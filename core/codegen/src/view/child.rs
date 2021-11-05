use quote::{quote, ToTokens};
use syn::{
  parse::{Parse, ParseStream},
  Result,
};

use super::HtmlSourceNode;

pub enum Child {
  RawBlock(syn::Block),
  Text(syn::ExprLit),
  View(HtmlSourceNode),
}

impl ToTokens for Child {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    match self {
      Self::RawBlock(block) => if block.stmts.len() == 1 {
        let first = &block.stmts[0];
        quote!(#first)
      } else {
        quote!(block)
      }
      .to_tokens(tokens),
      Self::Text(str) => quote! { etagere::view::Node::Text(#str) }.to_tokens(tokens),
      Self::View(view) => view.to_tokens(tokens),
    }
  }
}

impl Parse for Child {
  fn parse(input: ParseStream) -> Result<Self> {
    match input.parse::<HtmlSourceNode>() {
      Ok(view) => Ok(Self::View(view)),
      Err(_) => {
        if input.peek(syn::token::Brace) {
          let block = input.parse::<syn::Block>()?;
          Ok(Self::RawBlock(block))
        } else {
          let text = input.parse::<syn::ExprLit>()?;
          Ok(Self::Text(text))
        }
      }
    }
  }
}
