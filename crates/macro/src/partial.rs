use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

use crate::utils::FnInfo;

pub fn create_partial(input: TokenStream) -> TokenStream {
  let fn_info = FnInfo::new(input.clone(), parse_macro_input!(input as ItemFn));
  let FnInfo {
    block,
    generics,
    ident,
    input_fields,
    input_names,
    output,
    vis,
    ..
  } = fn_info;

  let impl_generics = generics.split_for_impl().0;

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

      #struct_params

      pub fn view #impl_generics ( #args_params ) #output {
        #block
      }
    }
  }
  .into()
}
