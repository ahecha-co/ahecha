use quote::{quote, ToTokens};
use syn::parse::Parse;

use super::node::Node;

#[derive(Debug, Default)]
pub struct Children {
  pub nodes: Vec<Node>,
  pub render_fragment: bool,
}

impl ToTokens for Children {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    if !self.nodes.is_empty() {
      let mut list = vec![];

      for node in self.nodes.iter() {
        match node {
          Node::Block(block) => list.push(quote!( #block )),
          _ => list.push(quote!( #node )),
        }
      }

      if self.render_fragment {
        quote! { [#(#list ),*] }.to_tokens(tokens);
      } else {
        quote! { #( .c( #list ) )* }.to_tokens(tokens);
      }
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

    Ok(Children {
      nodes,
      render_fragment: false,
    })
  }
}
