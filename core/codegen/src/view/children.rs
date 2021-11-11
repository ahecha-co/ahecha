use quote::{quote, ToTokens};
use syn::{
  parse::{Parse, ParseStream},
  Result,
};

use super::child::Child;

#[derive(Default)]
pub struct Children {
  pub nodes: Vec<Child>,
}

impl Children {
  pub fn new(nodes: Vec<Child>) -> Self {
    Children { nodes }
  }

  pub fn as_tokens(&self) -> proc_macro2::TokenStream {
    let children: Vec<_> = self.nodes.iter().map(|child| quote! { #child }).collect();

    match children.len() {
      0 => quote! { Option::<()>::None },
      1 => quote! { Some(ita_tuple_list::tuple_list!(#(#children),*)) },
      _ => quote! { Some(ita_tuple_list::tuple_list!(#(#children),*)) },
    }
  }
}

impl Parse for Children {
  fn parse(input: ParseStream) -> Result<Self> {
    let mut nodes = vec![];

    while !input.peek(syn::Token![<]) || !input.peek2(syn::Token![/]) {
      let child = input.parse::<Child>()?;
      nodes.push(child);
    }

    Ok(Self::new(nodes))
  }
}

impl ToTokens for Children {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    self.as_tokens().to_tokens(tokens);
  }
}
