use proc_macro::TokenStream;
use proc_macro_error::emit_error;
use quote::quote;
use syn::ItemFn;

use crate::utils::FnStruct;

pub fn create_document(f: ItemFn) -> TokenStream {
  let fn_struct: FnStruct = f.clone().into();

  let struct_name = fn_struct.name();
  let struct_str_name = struct_name.to_string();

  if struct_str_name != "Document" {
    emit_error!(
      struct_name.span(),
      "The name of the document must be `Document`"
    );
  }

  // TODO: validate that a doctype is provided
  // TODO: validate function arguments
  // TODO: destructure the page properties as the handler attributes, how? :shrug:

  quote! {
    #[cfg(feature = "backend")]
    #[allow(non_snake_case)]
    #[allow(dead_code)]
    #f
  }
  .into()
}
