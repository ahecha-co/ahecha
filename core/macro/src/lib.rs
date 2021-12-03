#![feature(proc_macro_span)]

extern crate proc_macro;

#[cfg(feature = "html-string-parser")]
use core::panic;

#[cfg(feature = "html-string-parser")]
use nom::error::ErrorKind;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;
use syn::{parse_macro_input, ExprCall, ItemFn};

use crate::html::node::HtmlNode;

mod api;
mod custom_element;
mod document;
mod html;
mod page;
mod partial;
mod routes;
mod utils;

#[proc_macro_attribute]
#[proc_macro_error]
pub fn api(_metadata: TokenStream, input: TokenStream) -> TokenStream {
  let f = parse_macro_input!(input as ItemFn);
  api::create_api(f)
}

#[proc_macro_attribute]
pub fn custom_element(_metadata: TokenStream, input: TokenStream) -> TokenStream {
  let f = parse_macro_input!(input as ItemFn);
  custom_element::create_custom_element(f)
}

#[proc_macro_attribute]
pub fn document(_metadata: TokenStream, input: TokenStream) -> TokenStream {
  let f = parse_macro_input!(input as ItemFn);
  document::create_document(f)
}

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
  use std::time::Instant;

  use proc_macro::Span;

  let start = Instant::now();
  let view = parse_macro_input!(input as HtmlNode);
  let source_file = Span::call_site().source_file().path().display().to_string();
  let line = Span::call_site().start().line;
  let res = quote! {
    {
      let res = #view;
      println!(
        "{}:{} | size {} Kilobytes",
        #source_file,
        #line,
        std::mem::size_of_val(&res) as f32 / 1000.0
      );
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

#[proc_macro_attribute]
#[proc_macro_error]
pub fn partial(_metadata: TokenStream, input: TokenStream) -> TokenStream {
  let f = parse_macro_input!(input as ItemFn);
  partial::create_partial(f)
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn page(metadata: TokenStream, input: TokenStream) -> TokenStream {
  let attributes = parse_macro_input!(metadata as syn::AttributeArgs);
  let f = parse_macro_input!(input as ItemFn);
  page::create_page(f, attributes)
}

#[proc_macro]
pub fn uri(input: TokenStream) -> TokenStream {
  let f = parse_macro_input!(input as ExprCall);
  let name = f.func.clone();
  let args = f.args;

  quote! {
    #name ::uri(#args)
  }
  .into()
}
