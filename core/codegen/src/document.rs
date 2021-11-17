use proc_macro::TokenStream;
use proc_macro_error::emit_error;
use quote::quote;

use crate::utils::FnStruct;

pub fn create_document(f: syn::ItemFn) -> TokenStream {
  let fn_struct: FnStruct = f.into();

  let vis = fn_struct.vis();
  let struct_name = fn_struct.name();
  let impl_generics = fn_struct.impl_generics();
  let ty_generics = fn_struct.type_generics();
  let where_clause = fn_struct.where_clause();
  let input_blocks = fn_struct.input_blocks();
  let input_fields = fn_struct.input_fields();
  let block = fn_struct.block();

  let struct_str_name = struct_name.to_string();

  if struct_str_name != "Document" {
    emit_error!(
      struct_name.span(),
      "The name of the document must be `Document`"
    );
  }

  // TODO: validate that a doctype is provided
  // TODO: validate function arguments
  // TODO: destructure the page properties as the handler attributes, how? :shrug:

  quote! {
    #vis struct #struct_name #impl_generics #input_blocks

    impl #ty_generics #struct_name #impl_generics #where_clause {
      pub fn handler(#input_fields) -> impl ahecha::view::Render {
        #block .render()
      }
    }
  }
  .into()
}
