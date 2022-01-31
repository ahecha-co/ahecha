use quote::{quote, ToTokens};
use syn::parse::Parse;

use super::node::Node;

#[derive(Debug, Default)]
pub struct Children {
  pub nodes: Vec<Node>,
}

impl ToTokens for Children {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    if self.nodes.is_empty() {
      quote! { vec![] }.to_tokens(tokens);
    } else {
      let mut list = vec![];

      for node in self.nodes.iter() {
        list.push(quote!( #node ));
      }

      quote! { vec![ #(#list),* ] }.to_tokens(tokens);
    }
  }
}

impl Parse for Children {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let mut nodes = Vec::new();

    while !input.is_empty() {
      match input.parse::<Node>() {
        Ok(node) => nodes.push(node),
        Err(_err) => {
          break;
        }
      }
    }

    Ok(Children { nodes })
  }
}