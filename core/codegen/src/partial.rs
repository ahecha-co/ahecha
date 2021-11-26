use proc_macro::TokenStream;
use quote::quote;

use crate::utils::FnStruct;

pub fn create_partial(f: syn::ItemFn) -> TokenStream {
  let fn_struct: FnStruct = f.into();
  let vis = fn_struct.vis();
  let struct_name = fn_struct.name();
  let view_fn = fn_struct.create_view();

  quote! {
    #[allow(non_snake_case)]
    #vis mod #struct_name {
      use super::*;

      #view_fn
    }
  }
  .into()
}
