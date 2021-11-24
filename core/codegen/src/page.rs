use proc_macro::TokenStream;
use proc_macro_error::emit_error;
use quote::quote;
use syn::AttributeArgs;

use crate::{
  page::attributes::PageAttributes,
  partial::create_partial_internal,
  routes::{generate_route_path, RouteType},
  utils::FnStruct,
};

mod attributes;

pub fn create_page(f: syn::ItemFn, attrs: AttributeArgs) -> TokenStream {
  let fn_struct: FnStruct = f.into();

  let vis = fn_struct.vis();
  let struct_name = fn_struct.name();
  let impl_generics = fn_struct.impl_generics();
  let ty_generics = fn_struct.type_generics();
  let where_clause = fn_struct.where_clause();
  let input_fields = fn_struct.input_fields();
  let block = fn_struct.block();

  let struct_str_name = struct_name.to_string();
  if struct_str_name.to_uppercase().chars().next().unwrap()
    != struct_str_name.chars().next().unwrap()
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

  let route = generate_route_path(RouteType::Page, struct_str_name, fn_struct.inputs());
  let uri = route.build_uri();
  let uri_input_fields = route.params();

  let partial = create_partial_internal(&fn_struct, false);

  quote! {
    #partial

    // When compiling all targets we get the partial struct definition, so we can skip this.
    // This is intended to use only for testing where all the features are enabled.
    #[derive(Debug)]
    #[cfg(all(feature = "frontend", not(feature = "backend")))]
    #vis struct #struct_name #impl_generics {}

    impl #ty_generics #struct_name #impl_generics #where_clause {
      pub fn uri(#uri_input_fields) -> String {
        #uri
      }

      #[cfg(feature = "backend")]
      pub fn handler(#input_fields) -> impl ahecha::view::Render {
        #document (#maybe_title, (), #block)
      }
    }
  }
  .into()
}
