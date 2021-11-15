use quote::{quote, ToTokens};

use crate::html::{attributes::Attributes, children::Children};

#[derive(Debug)]
pub struct HtmlPartial {
  pub attributes: Attributes,
  pub children: Children,
  pub name: String,
}

impl ToTokens for HtmlPartial {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let ident = syn::Ident::new(&self.name, proc_macro2::Span::call_site());
    let attributes = &self.attributes;
    let children = &self.children;

    let mut attrs = vec![];

    for (key, value) in attributes.attrs.iter() {
      let key = syn::Ident::new(key, proc_macro2::Span::call_site());
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
