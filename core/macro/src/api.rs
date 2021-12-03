use proc_macro::TokenStream;
use proc_macro_error::emit_error;
use quote::quote;

use crate::{routes::RouteType, utils::FnStruct};

pub fn create_api(f: syn::ItemFn) -> TokenStream {
  let fn_struct: FnStruct = f.into();

  let vis = fn_struct.vis();
  let struct_name = fn_struct.name();
  let return_type = fn_struct.return_type();

  let struct_str_name = struct_name.to_string();
  if struct_str_name.to_lowercase().chars().next().unwrap()
    != struct_str_name.chars().next().unwrap()
  {
    emit_error!(struct_name.span(), "Rest API functions must lower case");
  }

  let impl_generics = fn_struct.impl_generics();
  let ty_generics = fn_struct.type_generics();
  let where_clause = fn_struct.where_clause();
  let block = fn_struct.block();
  let input_fields = fn_struct.input_fields(quote!());
  let input_names = fn_struct
    .input_names()
    .iter()
    .map(|n| quote! {#n})
    .collect::<Vec<_>>();
  let (params_struct_definition, params_destructured) = if input_names.is_empty() {
    (quote!(), quote!())
  } else {
    (
      quote! {
        pub struct Params #impl_generics {
          #input_fields
        }
      },
      quote!( Params { #(#input_names),* }: Params #ty_generics ),
    )
  };

  let route_fn = fn_struct.create_route(RouteType::Api);
  let maybe_async = if fn_struct.is_async() {
    quote!(async)
  } else {
    quote!()
  };

  quote!(
    #[allow(non_camel_case_types)]
    #vis mod #struct_name {
      use super::*;

      #params_struct_definition

      pub #maybe_async fn handler #impl_generics
      (
        #input_fields
      ) #return_type
      #where_clause
        #block

      #route_fn
    }
  )
  .into()
}
