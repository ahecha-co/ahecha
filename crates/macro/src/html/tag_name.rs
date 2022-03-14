use std::fmt::Display;

use proc_macro2::{Ident, Span};
use syn::parse::Parse;

#[derive(Debug)]
pub struct TagName {
  name: String,
  span: Span,
}

impl TagName {
  pub fn span(&self) -> Span {
    self.span
  }
}

impl PartialEq for TagName {
  fn eq(&self, other: &Self) -> bool {
    self.name == other.name
  }
}

impl Display for TagName {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", &self.name)
  }
}

impl Parse for TagName {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let mut idents: Vec<Ident> = vec![input.parse()?];

    while input.peek(syn::Token!(-)) {
      input.parse::<syn::Token!(-)>()?;
      idents.push(input.parse()?);
    }

    Ok(TagName {
      span: idents[0].span(),
      name: idents
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<_>>()
        .join("-"),
    })
  }
}

// impl ToTokens for TagName {
//   fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
//     let name = &self.idents;
//     quote!(#(#name)-*).to_tokens(tokens);
//   }
// }
