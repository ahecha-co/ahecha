use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};

use super::ValidatorAttribute;

pub(crate) struct DateTimeValidator;

impl DateTimeValidator {
  pub(crate) fn new(ident: Ident, attr: ValidatorAttribute) -> Self {
    todo!()
  }
}

impl ToTokens for DateTimeValidator {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    todo!()
  }
}
