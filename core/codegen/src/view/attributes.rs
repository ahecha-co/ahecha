use proc_macro_error::emit_error;
use quote::{quote, ToTokens};
use syn::{
  ext::IdentExt,
  parse::{Parse, ParseStream},
  spanned::Spanned,
  Result,
};

use super::attribute::ViewAttribute;

pub type Attributes = Vec<ViewAttribute>;

#[derive(Default)]
pub struct ViewAttributes {
  pub attributes: Attributes,
}

impl ViewAttributes {
  pub fn new(attributes: Attributes) -> Self {
    Self { attributes }
  }

  pub fn for_custom_element<'c>(&self) -> CustomElementAttributes<'_> {
    CustomElementAttributes {
      attributes: &self.attributes,
    }
  }

  pub fn for_simple_element(&self) -> HtmlTagAttributes<'_> {
    HtmlTagAttributes {
      attributes: &self.attributes,
    }
  }

  pub fn parse(input: ParseStream, is_custom_element: bool) -> Result<Self> {
    let parsed_self = input.parse::<Self>()?;

    let new_attributes: Attributes = parsed_self
      .attributes
      .iter()
      .filter_map(
        |attribute| match attribute.clone().validate(is_custom_element) {
          Ok(x) => Some(x),
          Err(err) => {
            emit_error!(err.span(), "Invalid attribute: {}", err);
            None
          }
        },
      )
      .collect();

    Ok(ViewAttributes::new(new_attributes))
  }
}

impl Parse for ViewAttributes {
  fn parse(input: ParseStream) -> Result<Self> {
    let mut attributes: Vec<ViewAttribute> = vec![];
    while input.peek(syn::Ident::peek_any) {
      let attribute = input.parse::<ViewAttribute>()?;
      let ident = attribute.ident();
      if attributes.contains(&attribute) {
        emit_error!(
          ident.span(),
          "There is a previous definition of the {} attribute",
          quote!(#ident)
        );
      }

      attributes.push(attribute);
    }

    Ok(ViewAttributes::new(attributes))
  }
}

pub struct CustomElementAttributes<'a> {
  attributes: &'a Attributes,
}

impl<'a> ToTokens for CustomElementAttributes<'a> {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let attrs: Vec<_> = self
      .attributes
      .iter()
      .map(|attribute| {
        let ident = attribute.ident();
        let value = attribute.value_tokens();

        quote! {
          #ident : #value
        }
      })
      .collect();

    quote!( { #(#attrs),* } ).to_tokens(tokens);
  }
}

pub struct HtmlTagAttributes<'a> {
  attributes: &'a Attributes,
}

impl<'a> ToTokens for HtmlTagAttributes<'a> {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    if self.attributes.is_empty() {
      quote!(()).to_tokens(tokens);
    } else {
      let attrs: Vec<_> = self
        .attributes
        .iter()
        .map(|attribute| {
          let mut iter = attribute.ident().iter();
          let first_word = iter.next().unwrap().unraw();
          let ident = iter.fold(first_word.to_string(), |acc, curr| {
            format!("{}-{}", acc, curr.unraw())
          });

          let value = attribute.value_tokens();
          quote! {
            (#ident,  #value)
          }
        })
        .collect();

      let hashmap_declaration = quote! {
        [ #(#attrs),*]
      };

      hashmap_declaration.to_tokens(tokens);
    }
  }
}
