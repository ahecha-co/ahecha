use proc_macro::TokenStream;
use proc_macro_error::emit_error;
use quote::quote;

use crate::utils::FnStruct;

pub fn create_page(f: syn::ItemFn) -> TokenStream {
  let fn_struct: FnStruct = f.into();

  let vis = fn_struct.vis();
  let struct_name = fn_struct.name();
  let block = fn_struct.block();
  let impl_generics = fn_struct.impl_generics();
  let ty_generics = fn_struct.type_generics();
  let where_clause = fn_struct.where_clause();
  let input_blocks = fn_struct.input_blocks();

  let input_readings = fn_struct.input_readings();

  let struct_str_name = struct_name.to_string();
  if struct_str_name.to_uppercase().chars().next().unwrap()
    != struct_str_name.chars().next().unwrap()
  {
    emit_error!(struct_name.span(), "Pages must start with a upper letter");
  }

  if !struct_str_name.ends_with("Partial") {
    emit_error!(
      struct_name.span(),
      "Pages must have the `Page` suffix, example: `{}Page`",
      struct_str_name
    );
  }

  quote! {
    #[derive(Debug)]
    #vis struct #struct_name #impl_generics #input_blocks

    impl #ty_generics #struct_name #impl_generics #where_clause {
      pub fn route() -> &'static str {
        #struct_name::route()
      }
    }

    impl #impl_generics ahecha::view::Render for #struct_name #ty_generics #where_clause {
      fn render_into<W: std::fmt::Write>(self, w: &mut W) -> ::std::fmt::Result {
        let result = {
          #input_readings
          #block
        }.render();

        write!(w, "{}", result)
      }
    }

    impl #impl_generics Into<String> for #struct_name #ty_generics #where_clause {
      fn into(self) -> String {
        self.render()
      }
    }
  }
  .into()
}
