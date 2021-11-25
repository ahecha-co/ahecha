use proc_macro::TokenStream;
use proc_macro_error::emit_error;
use quote::quote;
use syn::AttributeArgs;

use crate::{
  page::attributes::PageAttributes,
  routes::{generate_route_path, RouteType},
  utils::FnStruct,
};

mod attributes;

pub fn create_page(f: syn::ItemFn, attrs: AttributeArgs) -> TokenStream {
  let fn_struct: FnStruct = f.into();

  let struct_name = fn_struct.name();

  let struct_str_name = struct_name.to_string();
  if !struct_str_name
    .chars()
    .next()
    .expect("Page must have a name")
    .is_uppercase()
  {
    emit_error!(struct_name.span(), "Pages must start with a upper letter");
  }

  if !struct_str_name.ends_with("Page") {
    emit_error!(
      struct_name.span(),
      "Pages must have the `Page` suffix, example: `{}Page`",
      struct_str_name
    );
  }

  let attributes = PageAttributes::from_meta(&attrs);
  let document = attributes.document;
  let maybe_title = {
    if let Some(title) = attributes.title {
      quote! { Some(#title) }
    } else {
      quote!(None)
    }
  };

  let input_names = fn_struct
    .input_names()
    .iter()
    .map(|n| quote! {#n})
    .collect::<Vec<_>>();
  let (params, params_ref) = if input_names.is_empty() {
    (quote!(), quote!())
  } else {
    (quote!(params: Params,), quote!(params))
  };

  let view_fn = fn_struct.create_view();

  let vis = fn_struct.vis();

  let route = generate_route_path(RouteType::Page, struct_str_name, fn_struct.inputs());
  let uri = route.build_uri();
  let uri_input_fields = route.params();

  quote! {
    #[allow(non_snake_case)]
    #vis mod #struct_name {
      use super::*;

      #[cfg(feature = "backend")]
      pub fn handler( #params ) -> impl ahecha::view::RenderString {
        #document ( #maybe_title , (), view( #params_ref ))
      }

      pub fn uri( #uri_input_fields ) -> String {
        #uri
      }

      #view_fn
    }
  }
  .into()
}
