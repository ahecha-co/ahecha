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
          #block .into()
        }
        .to_tokens(tokens);
      }
      AttributeValue::Lit(s) => {
        quote!(ahecha::html::AttributeValue::String(#s .to_string())).to_tokens(tokens)
      }
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
  pub extended: Vec<Ident>,
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
      extended: vec![],
      key: Ident::new("", Span::call_site()),
      value: AttributeValue::Lit(Lit::Str(LitStr::new("", Span::call_site()))),
    }
  }
}

impl Parse for Attribute {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let mut extended = vec![];
    let key = if input.peek(syn::token::Type) {
      input.parse::<syn::token::Type>()?;
      Ident::new("type", Span::call_site())
    } else if input.peek(syn::token::For) {
      input.parse::<syn::token::For>()?;
      Ident::new("for", Span::call_site())
    } else if input.peek(syn::Ident) && input.peek2(syn::Token![-]) {
      let kind_ident = input.parse::<syn::Ident>()?;

      while input.peek(syn::Token![-]) {
        input.parse::<syn::Token![-]>()?;
        extended.push(input.parse::<syn::Ident>()?);
      }

      // if !["aria", "data"].contains(&kind_ident.to_string().as_str()) {
      //   return Err(syn::Error::new(
      //     Span::call_site(),
      //     "Unsupported attribute kind",
      //   ));
      // }

      kind_ident
    } else {
      input.parse()?
    };

    let value = if input.peek(syn::Token![=]) {
      input.parse::<syn::Token![=]>()?;
      input.parse()?
    } else {
      AttributeValue::Lit(Lit::Bool(LitBool::new(true, Span::call_site())))
    };

    Ok(Attribute {
      extended,
      key,
      value,
    })
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
    if self.attrs.is_empty() {
      quote!(vec![])
    } else {
      let mut list = vec![];

      for Attribute {
        extended,
        key,
        value,
      } in self.attrs.iter()
      {
        let key = vec![key.clone()]
          .into_iter()
          .chain(extended.clone())
          .map(|i| i.to_string())
          .collect::<Vec<_>>()
          .join("-");
        list.push(quote! { (#key.to_owned(), #value) })
      }

      quote!(vec![ #(#list),* ])
    }
    .to_tokens(tokens);
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
