pub mod attribute;

use std::fmt::Debug;

use quote::{quote, ToTokens};
use syn::{ext::IdentExt, parse::Parse, token::Brace};

use self::attribute::Attribute;

#[derive(Debug, Default)]
pub struct Attributes {
  pub attrs: Vec<Attribute>,
}

impl From<Vec<Attribute>> for Attributes {
  fn from(attrs: Vec<Attribute>) -> Self {
    Self { attrs }
  }
}

impl From<Option<Vec<Attribute>>> for Attributes {
  fn from(attrs: Option<Vec<Attribute>>) -> Self {
    if let Some(attrs) = attrs {
      Self::from(attrs)
    } else {
      Self::default()
    }
  }
}

impl ToTokens for Attributes {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    if !self.attrs.is_empty() {
      let mut list = vec![];

      for attr in self.attrs.iter() {
        list.push(quote!( #attr ));
      }

      quote!( #(#list)* ).to_tokens(tokens);
    }
  }
}

impl Parse for Attributes {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let mut attrs = vec![];

    while input.peek(syn::Ident::peek_any) || input.peek(Brace) {
      let attr = input.parse()?;
      attrs.push(attr);
    }

    Ok(Attributes { attrs })
  }
}
