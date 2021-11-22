use quote::{quote, ToTokens};

use crate::html::{
  attributes::{Attribute, Attributes},
  children::Children,
};

#[derive(Debug)]
pub struct HtmlPartial {
  pub attributes: Attributes,
  pub children: Children,
  pub name: syn::Ident,
}

impl ToTokens for HtmlPartial {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let ident = &self.name;
    let attributes = &self.attributes;
    let children = &self.children;

    let mut attrs = vec![];

    for Attribute { key, value } in attributes.attrs.iter() {
      attrs.push(quote! {
        #key: #value
      });
    }

    if !children.nodes.is_empty() {
      attrs.push(quote!( children: #children ))
    }

    let element = quote!(
      ahecha::view::HtmlFragment {
        children: Some((
          #ident {
            #(#attrs,)*
          },
          ()
        )),
      }
    );
    element.to_tokens(tokens);
  }
}

impl ToString for HtmlPartial {
  fn to_string(&self) -> String {
    format!("<{}>...</{}>", self.name, self.name)
  }
}
