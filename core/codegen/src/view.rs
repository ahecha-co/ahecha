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

pub struct View {
  name: syn::Path,
  attributes: ViewAttributes,
  children: Children,
}

impl Parse for View {
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

    Ok(View {
      name: open_tag.name,
      attributes: open_tag.attributes,
      children,
    })
  }
}

impl View {
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

impl ToTokens for View {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let name = &self.name;

    let declaration = if self.is_custom_element() {
      let attrs = self.attributes.for_custom_element(&self.children);
      quote!( #name #attrs )
    } else {
      let attrs = self.attributes.for_simple_element();
      let children_tuple = self.children.as_option_of_tuples_tokens();

      quote! {
        etagere::view::HtmlTag {
          tag_name: stringify!(#name),
          attributes: #attrs,
          children: #children_tuple,
        }
      }
    };

    declaration.to_tokens(tokens);
  }
}
