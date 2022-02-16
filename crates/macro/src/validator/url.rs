use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};

use super::ValidatorAttribute;

pub(crate) struct UrlValidator;

impl UrlValidator {
  pub(crate) fn new(ident: Ident, attr: ValidatorAttribute) -> Self {
    todo!()
  }
}

impl ToTokens for UrlValidator {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    todo!()
  }
}
