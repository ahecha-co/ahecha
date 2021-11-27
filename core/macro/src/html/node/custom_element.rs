use convert_case::{Case, Casing};
use quote::{quote, ToTokens};

use crate::html::{
  attributes::{Attribute, Attributes},
  children::Children,
};

#[derive(Debug)]
pub struct HtmlCustomElement {
  pub attributes: Attributes,
  pub children: Children,
  pub name: syn::Ident,
}

impl ToTokens for HtmlCustomElement {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let ident = &self.name;
    let name = &self.name.to_string().to_case(Case::Kebab);
    let attributes = &self.attributes;
    let children = &self.children;

    let mut attrs = vec![];

    for Attribute {
      extended: _,
      key,
      value,
    } in attributes.attrs.iter()
    {
      attrs.push(quote! {
        #key: #value
      });
    }

    if !children.nodes.is_empty() {
      attrs.push(quote!( children: #children ))
    }

    let param = if attrs.is_empty() {
      quote!()
    } else {
      quote!(
        #ident ::Params{
          #(#attrs,)*
        }
      )
    };

    let element = quote!(
      ahecha::view::HtmlElement {
        attributes: #attributes,
        children: Some((
          #ident ::view( #param ),
          ()
        )),
        name: #name,
      }
    );
    element.to_tokens(tokens);
  }
}

impl ToString for HtmlCustomElement {
  fn to_string(&self) -> String {
    format!("<{}>...</{}>", self.name, self.name)
  }
}
