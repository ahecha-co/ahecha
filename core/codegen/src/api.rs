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
  let impl_generics = fn_struct.impl_generics();
  let ty_generics = fn_struct.type_generics();
  let where_clause = fn_struct.where_clause();
  let input_blocks = fn_struct.input_blocks();
  let input_fields = fn_struct.input_fields();

  let struct_str_name = struct_name.to_string();
  if struct_str_name.to_lowercase().chars().next().unwrap()
    != struct_str_name.chars().next().unwrap()
  {
    emit_error!(struct_name.span(), "Rest API functions must lower case");
  }

  let route = generate_route_path(RouteType::Api, struct_str_name.clone(), fn_struct.inputs());
  let uri = route.build_uri();
  let mount_route = route.build(&fn_struct);
  let uri_input_fields = route.params();

  quote! {
    #[allow(non_camel_case_types)]
    #vis struct #struct_name #impl_generics #input_blocks

    impl #ty_generics #struct_name #impl_generics #where_clause {
      pub fn uri(#uri_input_fields) -> String {
        #uri
      }

      pub fn mount(#input_fields) #return_type {
        #mount_route
      }
    }
  }
  .into()
}
