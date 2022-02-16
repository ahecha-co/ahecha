use proc_macro::TokenStream;
use proc_macro_error::emit_error;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

use crate::{routes::RouteType, utils::FnInfo};

pub fn create_api(input: TokenStream) -> TokenStream {
  let fn_info = FnInfo::new(input.clone(), parse_macro_input!(input as ItemFn));
  let uri_fn = fn_info.uri(RouteType::Api);
  let FnInfo {
    ident,
    original_input,
    metadata_ident,
    vis,
    ..
  } = fn_info;

  let name = ident.to_string();
  if name.to_lowercase().chars().next().unwrap() != name.chars().next().unwrap() {
    emit_error!(ident.span(), "Rest API functions must lower case");
  }

  quote!(
    #original_input

    #[allow(non_camel_case_types)]
    #vis mod #metadata_ident {
      use super::*;

      #uri_fn
    }
  )
  .into()
}
