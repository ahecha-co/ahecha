use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};

use super::ValidatorAttribute;

pub(crate) struct DateValidator {
  ident: Ident,
  format: String,
  message: String,
}

impl DateValidator {
  pub(crate) fn new(ident: Ident, attr: ValidatorAttribute) -> Self {
    attr.allowed_attrs(vec!["format", "message"]);

    Self {
      format: attr.get("format").unwrap_or("YYYY-MM-DD".to_owned()),
      message: attr.get("message").unwrap_or(format!(
        "The provided date from `{}` doesn't have a valid format",
        ident.to_string(),
      )),
      ident,
    }
  }
}

impl ToTokens for DateValidator {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let field = self.ident.to_string();
    let message = &self.message;
    let _format = &self.format;

    // TODO: date format validator, this is just a very naive implementation
    quote!({
      match value[#field] {
        serde_json::Value::String(v) => {
          let parts = v.split('-');

          if parts.len() == 3 {
          } else {
            errors.push(ahecha::validate::Error(#message));
          }
        }
        _ => errors.push(ahecha::validate::Error(#message)),
      }
    })
    .to_tokens(tokens);
  }
}
