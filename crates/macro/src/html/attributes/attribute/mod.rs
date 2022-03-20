use std::fmt::Debug;

use proc_macro2::Ident;
use quote::ToTokens;
use syn::{ext::IdentExt, parse::Parse};

use self::{block::AttributeBlock, key_value::AttributeKeyValue};

pub mod block;
pub mod key_value;

#[derive(Debug)]
pub enum Attribute {
  Block(AttributeBlock),
  KeyValue(AttributeKeyValue),
}

impl Attribute {
  pub fn key_exists(&self, key: &str) -> bool {
    match self {
      Attribute::Block(_) => false,
      Attribute::KeyValue(key_value) => key_value.key.to_string() == key,
    }
  }
}

impl Parse for Attribute {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    if input.peek(Ident::peek_any) {
      Ok(Attribute::KeyValue(input.parse()?))
    } else {
      Ok(Attribute::Block(input.parse()?))
    }
  }
}

impl ToTokens for Attribute {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    match self {
      Attribute::Block(block) => {
        block.to_tokens(tokens);
      }
      Attribute::KeyValue(key_value) => {
        key_value.to_tokens(tokens);
      }
    }
  }
}
