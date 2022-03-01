use quote::{quote, ToTokens};
use syn::parse::Parse;

use super::Node;

#[derive(Debug)]
pub enum HtmlDoctype {
  Html5(Box<Node>),
}

impl From<HtmlDoctype> for Node {
  fn from(element: HtmlDoctype) -> Self {
    Node::Doctype(element)
  }
}

impl ToTokens for HtmlDoctype {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    match self {
      HtmlDoctype::Html5(element) => {
        quote! {
          ahecha::html::Node::Document(ahecha::html::Doctype::Html5, ahecha::html::Children::default().set(#element))
        }
      }
    }
    .to_tokens(tokens);
  }
}

impl ToString for HtmlDoctype {
  fn to_string(&self) -> String {
    match self {
      HtmlDoctype::Html5(children) => {
        format!("<!doctype html>{}", children.to_string())
      }
    }
  }
}

impl Parse for HtmlDoctype {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    if !input.peek(syn::Token![<]) || !input.peek2(syn::Token![!]) {
      return Err(syn::Error::new(input.span(), "expected <!DOCTYPE html>"));
    }

    input.parse::<syn::Token![<]>()?;
    input.parse::<syn::Token![!]>()?;
    let _doctype = input.parse::<syn::Ident>()?;
    let _html = input.parse::<Option<syn::Ident>>()?;
    // TODO validate that the doctype is html5
    input.parse::<syn::Token![>]>()?;
    let children = input.parse()?;
    Ok(HtmlDoctype::Html5(children))
  }
}
