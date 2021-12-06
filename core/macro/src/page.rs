use proc_macro::TokenStream;
use proc_macro_error::emit_error;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, ItemFn};

use crate::{page::attributes::PageAttributes, routes::RouteType, utils::FnInfo};

mod attributes;

pub fn create_page(attrs: AttributeArgs, input: TokenStream) -> TokenStream {
  let fn_info = FnInfo::new(input.clone(), parse_macro_input!(input as ItemFn));
  let uri_fn = fn_info.uri(RouteType::Page);
  let FnInfo {
    ident,
    inputs,
    input_names,
    is_async,
    is_ident_capitalized,
    original_input,
    metadata_ident,
    vis,
    ..
  } = fn_info;

  if !is_ident_capitalized {
    emit_error!(ident.span(), "Pages must start with a upper letter");
  }

  if !ident.to_string().ends_with("Page") {
    emit_error!(
      ident.span(),
      "Pages must have the `Page` suffix, example: `{}Page`",
      ident.to_string()
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

  let maybe_async = if is_async { quote!(async) } else { quote!() };

  quote! {
    #original_input

    #[allow(non_snake_case)]
    #vis mod #metadata_ident {
      use super::*;

      #[cfg(feature = "backend")]
      pub #maybe_async fn handler( #inputs ) -> impl ahecha::html::RenderString {
        #document ( #maybe_title , (), #ident ( #(#input_names),* ))
      }

      #uri_fn
    }
  }
  .into()
}
