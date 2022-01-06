#![feature(proc_macro_span)]

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ExprCall};

use crate::html::node::Node;

mod api;
mod custom_element;
mod document;
mod html;
mod model;
mod page;
mod partial;
mod routes;
mod utils;
mod validator;

#[proc_macro_attribute]
#[proc_macro_error]
pub fn api(_metadata: TokenStream, input: TokenStream) -> TokenStream {
  api::create_api(input)
}

#[proc_macro_attribute]
pub fn custom_element(_metadata: TokenStream, input: TokenStream) -> TokenStream {
  custom_element::create_custom_element(input)
}

#[proc_macro_attribute]
pub fn document(_metadata: TokenStream, input: TokenStream) -> TokenStream {
  document::create_document(input)
}

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
  let view = parse_macro_input!(input as Node);
  quote!(#view).into()
}

#[proc_macro_derive(
  Model,
  attributes(table_name, primary_key, one_to_one, one_to_many, belongs_to)
)]
#[proc_macro_error]
pub fn model(input: TokenStream) -> TokenStream {
  model::create_model(parse_macro_input!(input as DeriveInput)).into()
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn partial(_metadata: TokenStream, input: TokenStream) -> TokenStream {
  partial::create_partial(input)
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn page(metadata: TokenStream, input: TokenStream) -> TokenStream {
  page::create_page(parse_macro_input!(metadata as syn::AttributeArgs), input)
}

#[proc_macro]
pub fn uri(input: TokenStream) -> TokenStream {
  // TODO: Fix it, get the function path/name and convert it to __{:name}_metadata
  let f = parse_macro_input!(input as ExprCall);
  let name = f.func.clone();
  let args = f.args;

  quote! {
    #name ::uri(#args)
  }
  .into()
}

#[proc_macro_derive(Validate, attributes(validate))]
#[proc_macro_error]
pub fn validator(input: TokenStream) -> TokenStream {
  validator::create_validator(input)
}
