use proc_macro::TokenStream;
use proc_macro_error::emit_error;
use quote::quote;

use crate::utils::FnStruct;

pub(crate) fn create_partial_internal(
  fn_struct: &FnStruct,
  is_partial: bool,
) -> proc_macro2::TokenStream {
  let vis = fn_struct.vis();
  let struct_name = fn_struct.name();

  if is_partial {
    let struct_str_name = struct_name.to_string();
    if !struct_str_name
      .chars()
      .next()
      .expect("Partial must have a name")
      .is_uppercase()
    {
      emit_error!(
        struct_name.span(),
        "Partials must start with a upper letter"
      );
    }
  }

  let view_fn = fn_struct.create_view();

  quote! {
    #[allow(non_snake_case)]
    #vis mod #struct_name {
      use super::*;

      #view_fn
    }
  }
}

pub fn create_partial(f: syn::ItemFn) -> TokenStream {
  let fn_struct = f.into();
  create_partial_internal(&fn_struct, true).into()
}
