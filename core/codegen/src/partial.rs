use proc_macro::TokenStream;
use proc_macro_error::emit_error;
use quote::quote;

use crate::utils::FnStruct;

pub(crate) fn create_partial_internal(
  fn_struct: &FnStruct,
  is_partial: bool,
) -> proc_macro2::TokenStream {
  let vis = fn_struct.vis();
  let struct_name = fn_struct.name();
  let block = fn_struct.block();
  let impl_generics = fn_struct.impl_generics();
  let ty_generics = fn_struct.type_generics();
  let where_clause = fn_struct.where_clause();
  let input_blocks = fn_struct.input_blocks();

  let input_readings = fn_struct.input_readings();

  if is_partial {
    let struct_str_name = struct_name.to_string();
    if struct_str_name.to_uppercase().chars().next().unwrap()
      != struct_str_name.chars().next().unwrap()
    {
      emit_error!(
        struct_name.span(),
        "Partials must start with a upper letter"
      );
    }

    if !struct_str_name.ends_with("Partial") {
      emit_error!(
        struct_name.span(),
        "Partials must have the `Partial` suffix, example: `{}Partial`",
        struct_str_name
      );
    }
  }

  quote! {
    #[derive(Debug)]
    #vis struct #struct_name #impl_generics #input_blocks

    // #[cfg(feature = "frontend")]
    // impl #impl_generics ahecha::view::RenderNode for #struct_name #ty_generics #where_clause {
    //   fn render(&self) -> web_sys::Node {
    //     return {
    //       #input_readings
    //       #block
    //     }.render()
    //   }
    // }

    impl #impl_generics ahecha::view::RenderString for #struct_name #ty_generics #where_clause {
      fn render_into<W: std::fmt::Write>(self, w: &mut W) -> ::std::fmt::Result {
        let result = {
          #input_readings
          #block
        }.render();

        write!(w, "{}", result)
      }
    }
  }
}

pub fn create_partial(f: syn::ItemFn) -> TokenStream {
  let fn_struct = f.into();
  create_partial_internal(&fn_struct, true).into()
}
