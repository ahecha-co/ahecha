use proc_macro2::{Ident, TokenStream};
use proc_macro_error::emit_error;
use quote::quote;
use syn::{
  punctuated::Punctuated, spanned::Spanned, token::Comma, Block, FnArg, Generics, ItemFn, Lifetime,
  Pat, ReturnType, Visibility,
};

use crate::routes::{generate_route_path, RoutePart, RoutePartDynamic, RouteType};

pub struct FnInfo {
  pub block: Box<Block>,
  pub generics: Generics,
  pub ident: Ident,
  pub input_fields: TokenStream,
  pub input_names: Vec<Pat>,
  pub inputs: Punctuated<FnArg, Comma>,
  pub is_async: bool,
  pub is_ident_capitalized: bool,
  pub lifetimes: Vec<Lifetime>,
  pub metadata_ident: Ident,
  pub original_input: proc_macro2::TokenStream,
  pub output: ReturnType,
  pub vis: Visibility,
}

impl FnInfo {
  pub fn new(input: proc_macro::TokenStream, item_fn: ItemFn) -> Self {
    let original_input = input.clone().into();

    let vis = &item_fn.vis;
    let is_ident_capitalized = item_fn
      .sig
      .ident
      .to_string()
      .chars()
      .next()
      .expect("fn name")
      .is_uppercase();

    let lifetimes = item_fn
      .sig
      .generics
      .lifetimes()
      .map(|l| l.lifetime.clone())
      .collect::<Vec<_>>();

    let generics = item_fn.sig.generics.clone();

    let input_names = item_fn
      .sig
      .inputs
      .iter()
      .filter_map(|argument| match argument {
        syn::FnArg::Typed(typed) => Some(typed),
        syn::FnArg::Receiver(rec) => {
          emit_error!(rec.span(), "Don't use `self` on components");
          None
        }
      })
      .map(|value| *value.pat.clone())
      .collect();

    let input_fields = if !item_fn.sig.inputs.is_empty() {
      let input_names: Vec<_> = item_fn
        .sig
        .inputs
        .iter()
        .map(|field| quote!(pub #field))
        .collect();
      quote!(#(#input_names),*,)
    } else {
      quote!()
    };

    let metadata_ident = Ident::new(
      format!("__{}_metadata", item_fn.sig.ident.to_string()).as_str(),
      item_fn.span(),
    );

    Self {
      block: item_fn.block,
      generics,
      ident: item_fn.sig.ident,
      input_fields,
      input_names,
      inputs: item_fn.sig.inputs,
      is_async: item_fn.sig.asyncness.is_some(),
      is_ident_capitalized,
      lifetimes,
      metadata_ident,
      original_input,
      output: item_fn.sig.output,
      vis: vis.clone(),
    }
  }

  pub fn uri(&self, route_type: RouteType) -> proc_macro2::TokenStream {
    let route = generate_route_path(route_type, self.ident.to_string(), &self.inputs);
    let uri = route.build_uri();
    let params = route
      .parts
      .iter()
      .filter_map(|part| match part {
        RoutePart::Static(_) => None,
        RoutePart::Dynamic(d) => Some(d),
      })
      .map(|RoutePartDynamic { ident, .. }| quote!( #ident ))
      .collect::<Vec<_>>();
    let (generics, arguments) = if params.is_empty() {
      (quote!(), quote!())
    } else {
      let abc = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
      let generic_idents = (0..params.len())
        .map(|i| Ident::new(&abc.chars().nth(i).unwrap().to_string(), self.ident.span()))
        .collect::<Vec<_>>();

      let generics = generic_idents
        .iter()
        .map(|g| quote!( #g: ToString ))
        .collect::<Vec<_>>();

      let arguments = generic_idents
        .iter()
        .enumerate()
        .map(|(i, g)| {
          let ident = params[i].clone();

          quote!(#ident : #g)
        })
        .collect::<Vec<_>>();

      (quote!(< #(#generics,)* >), quote!(#(#arguments,)*))
    };

    quote!(
      pub fn uri #generics ( #arguments ) -> String {
        #uri
      }
    )
  }
}
// impl FnStruct {
//   pub fn create_view(&self) -> proc_macro2::TokenStream {
//     let lifetimes = self
//       ._f
//       .sig
//       .generics
//       .lifetimes()
//       .map(|l| {
//         let lifetime = l.lifetime.clone();
//         quote!(#lifetime)
//       })
//       .collect::<Vec<_>>();
//     let impl_generics = self.impl_generics();
//     let ty_generics = self.type_generics();
//     let where_clause = self.where_clause();
//     let block = self.block();
//     let input_fields = self.input_fields(quote!(pub));
//     let input_names = self
//       .input_names()
//       .iter()
//       .map(|n| quote! {#n})
//       .collect::<Vec<_>>();
//     let (params_struct_definition, params_destructured) = if input_names.is_empty() {
//       (quote!(), quote!())
//     } else {
//       (
//         quote! {
//           pub struct Params #impl_generics {
//             #input_fields
//           }
//         },
//         quote!( Params { #(#input_names),* }: Params #ty_generics ),
//       )
//     };

//     let lifetimes = if lifetimes.is_empty() {
//       quote!()
//     } else {
//       quote!( + #(#lifetimes)+* )
//     };

//     quote!(
//       #params_struct_definition

//       pub fn view #impl_generics
//       (
//         #params_destructured
//       ) -> impl ahecha::html::RenderString #lifetimes #where_clause {
//         #block
//       }
//     )
//   }

//   pub fn create_route(&self, route_type: RouteType) -> TokenStream {
//     let route = generate_route_path(route_type, self.name().to_string(), self.inputs());
//     let uri = route.build_uri();
//     let uri_input_fields = route.params();

//     quote!(
//       pub fn uri( #uri_input_fields ) -> String {
//         #uri
//       }
//     )
//   }

//   pub fn type_generics(&self) -> TypeGenerics {
//     self._f.sig.generics.split_for_impl().1
//   }

//   pub fn where_clause(&self) -> Option<&WhereClause> {
//     self._f.sig.generics.split_for_impl().2
//   }
// }
