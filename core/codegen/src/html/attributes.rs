use std::fmt::Debug;

use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::{ext::IdentExt, parse::Parse, Block, Lit, LitBool, LitStr};

pub enum AttributeValue {
  Block(Block),
  Lit(Lit),
}

impl ToTokens for AttributeValue {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    match self {
      AttributeValue::Block(block) => {
        quote! {
          #block
        }
        .to_tokens(tokens);
      }
      AttributeValue::Lit(s) => quote!(#s).to_tokens(tokens),
    }
  }
}

impl Debug for AttributeValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      AttributeValue::Block(block) => {
        let block = block.clone();
        quote! {
          #block
        }
        .fmt(f)
      }
      AttributeValue::Lit(s) => quote! {#s}.fmt(f),
    }
  }
}

impl Parse for AttributeValue {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    if input.peek(syn::token::Brace) {
      Ok(AttributeValue::Block(input.parse::<Block>()?))
    } else {
      Ok(AttributeValue::Lit(input.parse::<Lit>()?))
    }
  }
}

pub struct Attribute {
  pub key: Ident,
  pub value: AttributeValue,
}

impl Debug for Attribute {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Attribute {{ key: {:?}, value: {:?} }}",
      self.key,
      self.value.to_token_stream().to_string()
    )
  }
}

impl Default for Attribute {
  fn default() -> Self {
    Self {
      key: Ident::new("", Span::call_site()),
      value: AttributeValue::Lit(Lit::Str(LitStr::new("", Span::call_site()))),
    }
  }
}

impl Parse for Attribute {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let key = input.parse()?;
    let value = if input.peek(syn::Token![=]) {
      input.parse::<syn::Token![=]>()?;
      input.parse()?
    } else {
      AttributeValue::Lit(Lit::Bool(LitBool::new(true, Span::call_site())))
    };

    Ok(Attribute { key, value })
  }
}

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
    let mut list = quote! { () };

    for Attribute { key, value } in self.attrs.iter().rev() {
      let key = key.to_string();
      list = quote! { ((#key, #value), #list) }
    }

    list.to_tokens(tokens);
  }
}

impl Parse for Attributes {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let mut attrs = vec![];

    while input.peek(syn::Ident::peek_any) {
      let attr = input.parse()?;
      attrs.push(attr);
    }

    Ok(Attributes { attrs })
  }
}
