use proc_macro::TokenStream;
use proc_macro_error::emit_error;
use quote::quote;

use crate::utils::FnStruct;

pub fn create_custom_element(f: syn::ItemFn) -> TokenStream {
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
    emit_error!(
      struct_name.span(),
      "Custom elements must start with a upper letter"
    );
  }

  if struct_str_name.ends_with("Partial") || struct_str_name.ends_with("Page") {
    emit_error!(
      struct_name.span(),
      "Custom elements cannot end with `Partial` or `Page` suffix.",
    );
  }

  quote! {
    #[derive(Debug)]
    #vis struct #struct_name #impl_generics #input_blocks

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
        use ahecha::view::Render;
        let mut result = String::new();
        self.render_into(&mut result).unwrap();
        result
      }
    }
  }
  .into()
}
