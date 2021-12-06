use proc_macro::TokenStream;
use proc_macro_error::emit_error;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

use crate::utils::FnInfo;

pub fn create_document(input: TokenStream) -> TokenStream {
  let fn_info = FnInfo::new(input.clone(), parse_macro_input!(input as ItemFn));
  let FnInfo {
    ident,
    original_input,
    ..
  } = fn_info;

  if ident.to_string().as_str() != "Document" {
    emit_error!(ident.span(), "The name of the document must be `Document`");
  }

  // TODO: validate that a doctype is provided
  // TODO: validate function arguments
  // TODO: destructure the page properties as the handler attributes, how? :shrug:

  quote! {
    #[cfg(feature = "backend")]
    #original_input
  }
  .into()
}
