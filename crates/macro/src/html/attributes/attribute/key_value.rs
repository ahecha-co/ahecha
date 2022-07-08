use std::fmt::Debug;

use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::{parse::Parse, Block, Lit, LitBool, LitStr};

pub enum AttributeValue {
  Block(Block),
  Bool(bool),
  Lit(Lit),
}

impl ToTokens for AttributeValue {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    match self {
      AttributeValue::Block(block) => {
        if block
          .stmts
          .iter()
          .find(|s| match s {
            syn::Stmt::Local(_) => true,
            syn::Stmt::Item(_) => false,
            syn::Stmt::Expr(_) => false,
            syn::Stmt::Semi(_, _) => true,
          })
          .is_none()
        {
          let stmts = &block.stmts;
          quote! { /* stmts */#(#stmts)* }
        } else {
          quote! { /* block */ #block }
        }
        .to_tokens(tokens);
      }
      AttributeValue::Bool(s) => quote!(#s).to_tokens(tokens),
      AttributeValue::Lit(s) => quote!( /* Lit */ #s).to_tokens(tokens),
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
      AttributeValue::Bool(s) => quote! {#s}.fmt(f),
      AttributeValue::Lit(s) => quote! {#s}.fmt(f),
    }
  }
}

impl Parse for AttributeValue {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    if input.peek(syn::token::Brace) {
      Ok(AttributeValue::Block(input.parse::<Block>()?))
    } else {
      let value: Lit = input.parse::<Lit>()?;
      match value {
        Lit::Bool(LitBool { value, .. }) => Ok(AttributeValue::Bool(value)),
        _ => Ok(AttributeValue::Lit(value)),
      }
    }
  }
}

pub struct AttributeKeyValue {
  pub extended: Vec<Ident>,
  pub key: Ident,
  pub value: AttributeValue,
}

impl Debug for AttributeKeyValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Attribute {{ key: {:?}, value: {:?} }}",
      self.key,
      self.value.to_token_stream().to_string()
    )
  }
}

impl Default for AttributeKeyValue {
  fn default() -> Self {
    Self {
      extended: vec![],
      key: Ident::new("", Span::call_site()),
      value: AttributeValue::Lit(Lit::Str(LitStr::new("", Span::call_site()))),
    }
  }
}

impl ToTokens for AttributeKeyValue {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let key = vec![self.key.clone()]
      .into_iter()
      .chain(self.extended.clone())
      .map(|i| i.to_string())
      .collect::<Vec<_>>()
      .join("-");

    match &self.value {
      AttributeValue::Block(value) => {
        let value = if value
          .stmts
          .iter()
          .find(|s| match s {
            syn::Stmt::Local(_) => true,
            syn::Stmt::Item(_) => false,
            syn::Stmt::Expr(_) => false,
            syn::Stmt::Semi(_, _) => true,
          })
          .is_none()
        {
          let stmts = &value.stmts;
          quote! { /* stmts */#(#stmts)* }
        } else {
          quote! { /* block */ #value }
        };
        quote! { .dyn_attr( #key, || Some(#value) ) }
      }
      AttributeValue::Bool(value) => quote! { .bool_attr( #key, #value ) },
      AttributeValue::Lit(value) => quote! { .attr( #key, #value ) },
    }
    .to_tokens(tokens);
  }
}

impl Parse for AttributeKeyValue {
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
      AttributeValue::Bool(true)
    };

    Ok(AttributeKeyValue {
      extended,
      key,
      value,
    })
  }
}
