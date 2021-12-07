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
    block,
    generics,
    ident,
    inputs,
    input_fields,
    input_names,
    is_async,
    is_ident_capitalized,
    output,
    vis,
    ..
  } = fn_info;

  let impl_generics = generics.split_for_impl().0;

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
  let (struct_params, args_params) = if input_names.is_empty() {
    (quote!(), quote!())
  } else {
    (
      quote!(
        pub struct ViewParams #impl_generics {
          #input_fields
        }
      ),
      quote!( ViewParams { #(#input_names),* }: ViewParams ),
    )
  };

  quote! {
    #[allow(non_snake_case)]
    #vis mod #ident {
      use super::*;

      #[cfg(feature = "backend")]
      pub #maybe_async fn handler( #inputs ) -> impl ahecha::html::RenderString {
        #document ( #maybe_title , ahecha::html::Node::Fragment(vec![]), #block)
      }

      #struct_params

      pub fn view #impl_generics ( #args_params ) #output {
        #block
      }

      #uri_fn
    }
  }
  .into()
}
