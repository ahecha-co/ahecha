use proc_macro2::{Ident, TokenStream};
use proc_macro_error::emit_error;
use quote::quote;
use syn::{
  punctuated::Punctuated, spanned::Spanned, token::Comma, Block, FnArg, ImplGenerics, ItemFn,
  TypeGenerics, Visibility, WhereClause,
};

pub struct FnStruct {
  f: ItemFn,
}

impl FnStruct {
  pub fn vis(&self) -> &Visibility {
    &self.f.vis
  }

  pub fn name(&self) -> &Ident {
    &self.f.sig.ident
  }

  pub fn impl_generics(&self) -> ImplGenerics {
    self.f.sig.generics.split_for_impl().0
  }

  pub fn type_generics(&self) -> TypeGenerics {
    self.f.sig.generics.split_for_impl().1
  }

  pub fn where_clause(&self) -> Option<&WhereClause> {
    self.f.sig.generics.split_for_impl().2
  }

  pub fn inputs(&self) -> &Punctuated<FnArg, Comma> {
    &self.f.sig.inputs
  }

  pub fn block(&self) -> &Block {
    &self.f.block
  }

  pub fn input_blocks(&self) -> TokenStream {
    let input_blocks = if !self.inputs().is_empty() {
      let input_names: Vec<_> = self.inputs().iter().collect();
      let vis = &self.vis();
      quote!(#(#vis #input_names),*,)
    } else {
      quote!()
    };

    quote!(
      {
        #input_blocks
      }
    )
  }

  pub fn input_fields(&self) -> TokenStream {
    let input_fields = if !self.inputs().is_empty() {
      let input_names: Vec<_> = self.inputs().iter().collect();
      quote!(#(#input_names),*,)
    } else {
      quote!()
    };

    quote!(#input_fields)
  }

  pub fn input_readings(&self) -> TokenStream {
    let input_readings = if self.inputs().is_empty() {
      quote!()
    } else {
      let input_names: Vec<_> = self
        .inputs()
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
    let struct_name = self.name();

    quote! (
      let #struct_name {
        #input_readings
      } = self;
    )
  }
}

impl From<ItemFn> for FnStruct {
  fn from(f: ItemFn) -> Self {
    FnStruct { f }
  }
}
