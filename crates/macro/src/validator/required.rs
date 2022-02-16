use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};

use super::ValidatorAttribute;

pub(crate) struct RequiredValidator {
  field: Ident,
  message: String,
}

impl RequiredValidator {
  pub(crate) fn new(field: Ident, attr: Option<ValidatorAttribute>) -> Self {
    let message = if let Some(attr) = attr {
      attr.allowed_attrs(vec!["message"]);
      attr.get("message")
    } else {
      None
    };

    Self {
      message: message.unwrap_or(format!("The field {} is required", field.to_string())),
      field,
    }
  }
}

impl ToTokens for RequiredValidator {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let ident_str = self.field.to_string();
    let message = &self.message;

    quote!({
      if value.get(#ident_str).is_none() || value[#ident_str] == serde_json::Value::Null {
        errors.push((#ident_str, #message))
      }
    })
    .to_tokens(tokens);
  }
}
