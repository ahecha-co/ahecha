extern crate proc_macro;

use convert_case::{Case, Casing};
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;
use record::Record;
use syn::parse_macro_input;

use crate::html::node::Node;

mod html;
mod record;

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
  let view = parse_macro_input!(input as Node);
  quote!(
    #view
  )
  .into()
}

#[proc_macro_derive(Partial)]
pub fn partial(item: TokenStream) -> TokenStream {
  let input = parse_macro_input!(item as syn::ItemStruct);
  let name = input.ident;
  let id = name.to_string().to_case(Case::Kebab);
  let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
  quote!(
    impl #impl_generics ahecha::html::partials::PartialView for #name #ty_generics #where_clause {
      fn id(&self) -> String {
        #id .to_owned()
      }
    }
  )
  .into()
}

#[proc_macro_derive(Record, attributes(record))]
#[proc_macro_error]
pub fn record(item: TokenStream) -> TokenStream {
  let input = parse_macro_input!(item as syn::ItemStruct);
  let ident = input.ident.clone();
  let record = Record::new(input);
  quote!(
    impl #ident {
      #record
    }
  )
  .into()
}
