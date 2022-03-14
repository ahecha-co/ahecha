use quote::{quote, ToTokens};
use syn::parse::Parse;

use super::Node;
use crate::html::children::Children;

#[derive(Debug)]
pub struct HtmlFragment {
  pub children: Children,
}

impl From<HtmlFragment> for Node {
  fn from(fragment: HtmlFragment) -> Self {
    Node::Fragment(fragment)
  }
}

impl ToTokens for HtmlFragment {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let children = &self.children;
    let element = quote!(
      ahecha::html::Node::Fragment(
        #children,
      )
    );
    element.to_tokens(tokens);
  }
}

impl ToString for HtmlFragment {
  fn to_string(&self) -> String {
    "<>...</>".to_string()
  }
}

impl Parse for HtmlFragment {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    if input.peek(syn::Token![<]) && input.peek2(syn::Token![>]) {
      input.parse::<syn::Token![<]>()?;
      input.parse::<syn::Token![>]>()?;
      let children = Children::parse(input)?;
      input.parse::<syn::Token![<]>()?;
      input.parse::<syn::Token![/]>()?;
      input.parse::<syn::Token![>]>()?;
      Ok(HtmlFragment { children })
    } else {
      Err(syn::Error::new(input.span(), "expected <...>"))
    }
  }
}
