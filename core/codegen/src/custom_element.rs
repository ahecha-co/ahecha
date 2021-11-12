use proc_macro::TokenStream;
use proc_macro_error::emit_error;
use quote::quote;
use syn::spanned::Spanned;

pub fn create_custom_element(f: syn::ItemFn) -> TokenStream {
  let struct_name = f.sig.ident;
  let (impl_generics, ty_generics, where_clause) = f.sig.generics.split_for_impl();
  let inputs = f.sig.inputs;
  let block = f.block;
  let vis = f.vis;

  let input_blocks = if !inputs.is_empty() {
    let input_names: Vec<_> = inputs.iter().collect();
    quote!(#(#vis #input_names),*,)
  } else {
    quote!()
  };

  let input_blocks = quote!(
    {
      #input_blocks
    }
  );

  let input_readings = if inputs.is_empty() {
    quote!()
  } else {
    let input_names: Vec<_> = inputs
      .iter()
      .filter_map(|argument| match argument {
        syn::FnArg::Typed(typed) => Some(typed),
        syn::FnArg::Receiver(rec) => {
          emit_error!(rec.span(), "Don't use `self` on components");
          None
        }
      })
      .map(|value| {
        let pat = &value.pat;
        quote!(#pat)
      })
      .collect();

    quote!(
      #(#input_names),*,
    )
  };

  let input_readings = quote! (
    let #struct_name {
      #input_readings
    } = self;
  );

  quote! {
    #[derive(Debug)]
    #vis struct #struct_name #impl_generics #input_blocks

    impl #impl_generics ita::view::Render for #struct_name #ty_generics #where_clause {
      fn render_into<W: std::fmt::Write>(self, w: &mut W) -> ::std::fmt::Result {
        let result: String = {
          #input_readings
          #block
        };

        write!(w, "{}", result)
      }
    }

    impl #impl_generics Into<String> for #struct_name #ty_generics #where_clause {
      fn into(self) -> String {
        use ita::view::Render;
        let mut result = String::new();
        self.render_into(&mut result).unwrap();
        result
      }
    }
  }
  .into()
}
