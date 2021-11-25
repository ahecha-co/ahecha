use proc_macro::TokenStream;
use proc_macro_error::emit_error;
use quote::quote;

use crate::{
  routes::{generate_route_path, RouteType},
  utils::FnStruct,
};

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

  let route = generate_route_path(RouteType::Api, struct_str_name, fn_struct.inputs());
  let uri = route.build_uri();
  let uri_input_fields = route.params();

  let lifetimes = fn_struct
    ._f
    .sig
    .generics
    .lifetimes()
    .map(|l| {
      let lifetime = l.lifetime.clone();
      quote!(#lifetime)
    })
    .collect::<Vec<_>>();
  let impl_generics = fn_struct.impl_generics();
  let ty_generics = fn_struct.type_generics();
  let where_clause = fn_struct.where_clause();
  let block = fn_struct.block();
  let input_fields = fn_struct.input_fields(quote!(pub));
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

  let lifetimes = if lifetimes.is_empty() {
    quote!()
  } else {
    quote!( + #(#lifetimes)+* )
  };

  quote!(
    #[allow(non_camel_case_types)]
    #vis mod #struct_name {
      use super::*;

      #params_struct_definition

      pub fn handler #impl_generics
      (
        #params_destructured
      ) #return_type #lifetimes #where_clause {
        #block
      }

      pub fn uri( #uri_input_fields ) -> String {
        #uri
      }
    }
  )
  .into()
}
