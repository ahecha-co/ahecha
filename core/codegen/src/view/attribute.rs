use std::hash::{Hash, Hasher};

use quote::quote;
use syn::{
  ext::IdentExt,
  parse::{Parse, ParseStream},
  spanned::Spanned,
  Result,
};

pub type AttributeKey = syn::punctuated::Punctuated<syn::Ident, syn::Token![-]>;

pub enum ViewAttribute {
  Punned(AttributeKey),
  WithStr(AttributeKey, syn::LitStr),
  WithBlock(AttributeKey, syn::Block),
}

impl ViewAttribute {
  pub fn ident(&self) -> &AttributeKey {
    match self {
      Self::Punned(ident) | Self::WithStr(ident, _) | Self::WithBlock(ident, _) => ident,
    }
  }

  pub fn idents(&self) -> Vec<&syn::Ident> {
    self.ident().iter().collect::<Vec<_>>()
  }

  pub fn value_tokens(&self) -> proc_macro2::TokenStream {
    match self {
      Self::WithBlock(_, value) => {
        if value.stmts.len() == 1 {
          let first = &value.stmts[0];
          quote!(#first)
        } else {
          quote!(#value)
        }
      }
      Self::WithStr(_, value) => quote!(#value),
      Self::Punned(ident) => quote!(#ident),
    }
  }

  pub fn validate(self, is_custom_element: bool) -> Result<Self> {
    if is_custom_element {
      self.validate_custom_element()
    } else {
      self.validate_simple_element()
    }
  }

  fn validate_custom_element(self) -> Result<Self> {
    if self.idents().len() < 2 {
      Ok(self)
    } else {
      let alternative_name = self
        .idents()
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("_");

      let error_message = format!(
        "Can't use dash-delimited values on custom components. Did you mean `{}`?",
        alternative_name
      );

      Err(syn::Error::new(self.ident().span(), error_message))
    }
  }

  fn validate_simple_element(self) -> Result<Self> {
    match (&self, self.idents().len()) {
      (Self::Punned(ref key), len) if len > 1 => {
        let error_message = "Can't use punning with dash-delimited values";
        Err(syn::Error::new(key.span(), error_message))
      }
      _ => Ok(self),
    }
  }
}

impl PartialEq for ViewAttribute {
  fn eq(&self, other: &Self) -> bool {
    let _self_idents = self.idents();
    let self_idents: Vec<_> = _self_idents.iter().collect();
    let _other_idents = other.idents();
    let other_idents: Vec<_> = _other_idents.iter().collect();
    self_idents == other_idents
  }
}

impl Eq for ViewAttribute {}

impl Hash for ViewAttribute {
  fn hash<H: Hasher>(&self, state: &mut H) {
    let ident = self.idents();
    Hash::hash(&ident, state)
  }
}

impl Parse for ViewAttribute {
  fn parse(input: ParseStream) -> Result<Self> {
    let name = AttributeKey::parse_separated_nonempty_with(input, syn::Ident::parse_any)?;
    let not_punned = input.peek(syn::Token![=]);

    if !not_punned {
      return Ok(Self::Punned(name));
    }

    input.parse::<syn::Token![=]>()?;

    if input.peek(syn::token::Brace) {
      let value = input.parse::<syn::Block>()?;
      Ok(Self::WithBlock(name, value))
    } else {
      let value = input.parse::<syn::LitStr>()?;
      Ok(Self::WithStr(name, value))
    }
  }
}
