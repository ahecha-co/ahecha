use proc_macro2::TokenStream;
use proc_macro2::Ident;
use proc_macro_error::emit_error;
use quote::quote;

use crate::{routes::RouteType, utils::FnStruct};

pub fn create_api(f: syn::ItemFn) -> TokenStream {
  let fn_struct: FnStruct = f.into();

  let vis = fn_struct.vis();
  let struct_name = fn_struct.name();
  let mod_name = Ident::new(
    format!("__{}_metadata", &struct_name).as_str(),
    struct_name.span(),
  );

  let struct_str_name = struct_name.to_string();
  if struct_str_name.to_lowercase().chars().next().unwrap()
    != struct_str_name.chars().next().unwrap()
  {
    emit_error!(struct_name.span(), "Rest API functions must lower case");
  }

  let route_fn = fn_struct.create_route(RouteType::Api);

  quote!(
    #[allow(non_camel_case_types)]
    #vis mod #mod_name {
      use super::*;

      #route_fn
    }
  )
}
