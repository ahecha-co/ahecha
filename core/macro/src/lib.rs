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
  use std::time::Instant;

  use proc_macro::Span;

  let start = Instant::now();
  let view = parse_macro_input!(input as Node);
  let source_file = Span::call_site().source_file().path().display().to_string();
  let line = Span::call_site().start().line;
  let res = quote! {
    {
      let res = #view;
      // println!(
      //   "{}:{} | size {} Kilobytes",
      //   #source_file,
      //   #line,
      //   std::mem::size_of_val(&res) as f32 / 1000.0
      // );
      res
    }
  }
  .into();

  let elapsed = Instant::elapsed(&start);

  println!(
    "ahecha_macro::html! | took {} µs | {}:{}",
    elapsed.as_micros(),
    source_file,
    line,
  );

  res
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
