use convert_case::{Case, Casing};
use quote::{quote, ToTokens};

use crate::html::{attributes::Attributes, children::Children};

#[derive(Debug)]
pub struct HtmlCustomElement {
  pub attributes: Attributes,
  pub children: Children,
  pub name: String,
}

impl ToTokens for HtmlCustomElement {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let ident = syn::Ident::new(&self.name, proc_macro2::Span::call_site());
    let name = &self.name.to_case(Case::Kebab);
    let attributes = &self.attributes;
    let children = &self.children;

    let mut attrs = vec![];

    for (key, value) in attributes.attrs.iter() {
      let key = syn::Ident::new(&key, proc_macro2::Span::call_site());
      attrs.push(quote! {
        #key: #value
      });
    }

    if !children.nodes.is_empty() {
      attrs.push(quote!( children: #children ))
    }

    let element = quote!(
      ita::view::HtmlElement {
        attributes: #attributes,
        children: Some((
          #ident {
            #(#attrs,)*
          },
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