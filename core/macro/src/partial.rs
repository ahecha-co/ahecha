use proc_macro::TokenStream;
use quote::quote;

pub fn create_partial(input: TokenStream) -> TokenStream {
  let input: proc_macro2::TokenStream = input.into();

  quote! {
    #input
  }
  .into()
}
