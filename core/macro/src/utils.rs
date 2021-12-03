use proc_macro2::{Ident, TokenStream};
use proc_macro_error::emit_error;
use quote::quote;
use syn::{
  punctuated::Punctuated, spanned::Spanned, token::Comma, Block, FnArg, ImplGenerics, ItemFn, Pat,
  TypeGenerics, Visibility, WhereClause,
};

use crate::routes::{generate_route_path, RouteType};

pub struct FnStruct {
  pub _f: ItemFn,
}

impl FnStruct {
  pub fn vis(&self) -> &Visibility {
    &self._f.vis
  }

  pub fn block(&self) -> &Block {
    &self._f.block
  }

  pub fn create_view(&self) -> proc_macro2::TokenStream {
    let lifetimes = self
      ._f
      .sig
      .generics
      .lifetimes()
      .map(|l| {
        let lifetime = l.lifetime.clone();
        quote!(#lifetime)
      })
      .collect::<Vec<_>>();
    let impl_generics = self.impl_generics();
    let ty_generics = self.type_generics();
    let where_clause = self.where_clause();
    let block = self.block();
    let input_fields = self.input_fields(quote!(pub));
    let input_names = self
      .input_names()
      .iter()
      .map(|n| quote! {#n})
      .collect::<Vec<_>>();
    let (params_struct_definition, params_destructured) = if input_names.is_empty() {
      (quote!(), quote!())
    } else {
      (
        quote! {
          pub struct Params #impl_generics {
            #input_fields
          }
        },
        quote!( Params { #(#input_names),* }: Params #ty_generics ),
      )
    };

    let lifetimes = if lifetimes.is_empty() {
      quote!()
    } else {
      quote!( + #(#lifetimes)+* )
    };

    quote!(
      #params_struct_definition

      pub fn view #impl_generics
      (
        #params_destructured
      ) -> impl ahecha::html::RenderString #lifetimes #where_clause {
        #block
      }
    )
  }

  pub fn create_route(&self, route_type: RouteType) -> TokenStream {
    let route = generate_route_path(route_type, self.name().to_string(), self.inputs());
    let uri = route.build_uri();
    let uri_input_fields = route.params();

    quote!(
      pub fn uri( #uri_input_fields ) -> String {
        #uri
      }
    )
  }

  pub fn has_camel_case_name(&self, err_message: &str) -> bool {
    self
      .name()
      .to_string()
      .chars()
      .next()
      .expect(err_message)
      .is_uppercase()
  }

  pub fn impl_generics(&self) -> ImplGenerics {
    self._f.sig.generics.split_for_impl().0
  }

  pub fn inputs(&self) -> &Punctuated<FnArg, Comma> {
    &self._f.sig.inputs
  }

  pub fn input_fields(&self, vis: TokenStream) -> TokenStream {
    let input_fields = if !self.inputs().is_empty() {
      let input_names: Vec<_> = self.inputs().iter().map(|i| quote!(#vis #i)).collect();
      quote!(#(#input_names),*,)
    } else {
      quote!()
    };

    quote!(#input_fields)
  }

  pub fn input_names(&self) -> Vec<Pat> {
    self
      .inputs()
      .iter()
      .filter_map(|argument| match argument {
        syn::FnArg::Typed(typed) => Some(typed),
        syn::FnArg::Receiver(rec) => {
          emit_error!(rec.span(), "Don't use `self` on components");
          None
        }
      })
      .map(|value| *value.pat.clone())
      .collect()
  }

  pub fn is_async(&self) -> bool {
    self._f.sig.asyncness.is_some()
  }

  pub fn name(&self) -> &Ident {
    &self._f.sig.ident
  }

  pub fn return_type(&self) -> TokenStream {
    let return_type = &self._f.sig.output;
    quote!(#return_type)
  }

  pub fn type_generics(&self) -> TypeGenerics {
    self._f.sig.generics.split_for_impl().1
  }

  pub fn where_clause(&self) -> Option<&WhereClause> {
    self._f.sig.generics.split_for_impl().2
  }
}

impl From<ItemFn> for FnStruct {
  fn from(f: ItemFn) -> Self {
    FnStruct { _f: f }
  }
}
