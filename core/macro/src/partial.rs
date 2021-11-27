use proc_macro::TokenStream;
use proc_macro_error::emit_error;
use quote::quote;

use crate::utils::FnStruct;

pub fn create_partial(f: syn::ItemFn) -> TokenStream {
  let fn_struct: FnStruct = f.into();
  let vis = fn_struct.vis();
  let struct_name = fn_struct.name();
  let view_fn = fn_struct.create_view();

  let struct_str_name = struct_name.to_string();
  if !fn_struct.has_camel_case_name("Partial must have a name") {
    emit_error!(
      struct_name.span(),
      "Partials must start with a upper letter"
    );
  }

  if !struct_str_name.ends_with("Partial") {
    emit_error!(
      struct_name.span(),
      "Partials must have the `Partial` suffix, example: `{}Partial`",
      struct_str_name
    );
  }

  quote! {
    #[allow(non_snake_case)]
    #vis mod #struct_name {
      use super::*;

      #view_fn
    }
  }
  .into()
}
