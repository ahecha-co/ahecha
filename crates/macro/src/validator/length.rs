use proc_macro2::{Ident, TokenStream};
use proc_macro_error::emit_error;
use quote::{quote, ToTokens};

use super::ValidatorAttribute;

pub(crate) struct LengthValidator {
  field: Ident,
  max: Option<usize>,
  min: Option<usize>,
  message: String,
}

impl LengthValidator {
  pub(crate) fn new(field: Ident, attr: ValidatorAttribute) -> Self {
    attr.allowed_attrs(vec!["max", "min", "message"]);

    let max = if let Some(value) = attr.get("max") {
      match value.parse::<usize>() {
        Ok(value) => Some(value),
        _ => None,
      }
    } else {
      None
    };

    let min = if let Some(value) = attr.get("min") {
      match value.parse::<usize>() {
        Ok(value) => Some(value),
        _ => None,
      }
    } else {
      None
    };

    if max.is_none() && min.is_none() {
      emit_error!(
        field.span(),
        "Length validator needs at least one of `min` or `max` attributes"
      );
    }

    let message = if max.is_some() && min.is_some() {
      format!(
        "The field `{}` expects to have a value with length between {} and {}",
        field.to_string(),
        min.unwrap(),
        max.unwrap()
      )
    } else if max.is_some() {
      format!(
        "The field `{}` expects to have a value with a max length of {}",
        field.to_string(),
        max.unwrap()
      )
    } else {
      format!(
        "The field `{}` expects to have a value with a min length of {}",
        field.to_string(),
        min.unwrap()
      )
    };

    Self {
      field,
      max,
      min,
      message,
    }
  }
}

impl ToTokens for LengthValidator {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let ident_str = self.field.to_string();
    let message = &self.message;
    let mut res = vec![];

    if let Some(min) = self.min {
      res.push(quote!({
        match value[#ident_str] {
          serde_json::Value::String(str_value) => {
            if str_value.len() < #min {
              errors.push((#ident_str, #message));
            }
          }
          _ => emit_error!(self.ident.span(), "Length validator can be only used on string values"),
        }
      }));
    }

    if let Some(max) = self.max {
      res.push(quote!({
        match value[#ident_str] {
          serde_json::Value::String(str_value) => {
            if str_value.len() < #max {
              errors.push((#ident_str, #message));
            }
          }
          _ => emit_error!(self.ident.span(), "Length validator can be only used on string values"),
        }
      }));
    }

    quote!(#(#res)*).to_tokens(tokens);
  }
}
