use convert_case::{Case, Casing};
use quote::{quote, ToTokens};
use syn::{
  parse::{Parse, ParseStream},
  Result,
};

use self::{
  attributes::ViewAttributes,
  children::Children,
  tag::{ClosingTag, OpenTag},
};

mod attribute;
mod attributes;
mod child;
mod children;
mod tag;

pub struct HtmlSourceNode {
  pub name: syn::Path,
  attributes: ViewAttributes,
  children: Children,
}

impl Parse for HtmlSourceNode {
  fn parse(input: ParseStream) -> Result<Self> {
    let open_tag = input.parse::<OpenTag>()?;

    let children = if open_tag.self_closing {
      Children::default()
    } else {
      let children = input.parse::<Children>()?;
      let closing_tag = input.parse::<ClosingTag>()?;
      closing_tag.validate(&open_tag);
      children
    };

    Ok(HtmlSourceNode {
      name: open_tag.name,
      attributes: open_tag.attributes,
      children,
    })
  }
}

impl HtmlSourceNode {
  pub fn is_custom_element(&self) -> bool {
    match self.name.get_ident() {
      None => true,
      Some(ident) => {
        let name = ident.to_string();
        let first_letter = name.get(0..1).unwrap();
        first_letter.to_uppercase() == first_letter
      }
    }
  }
}

impl ToTokens for HtmlSourceNode {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let name = &self.name;

    let declaration = if self.is_custom_element() {
      let name_str = &self
        .name
        .segments
        .first()
        .unwrap()
        .ident
        .to_string()
        .to_case(Case::Kebab);
      let attrs = self.attributes.for_custom_element();
      let children = self.children.as_tokens();
      quote! {
        etagere::view::Node::CustomElement(
          Box::new(
            etagere::view::CustomElementWrapper {
              name: #name_str,
              custom_element: Box::new({
                let mut element = #name ::default();
                element.create(#attrs, #children);
                element
              }),
            }
          )
        )
      }
    } else {
      let attrs = self.attributes.for_simple_element();
      let children = self.children.as_tokens();

      quote! {
        etagere::view::Node::HtmlElement(
          Box::new(
            etagere::view::HtmlElement {
              name: stringify!(#name),
              attributes: #attrs,
              children: #children,
            }
          )
        )
      }
    };

    declaration.to_tokens(tokens);
  }
}
