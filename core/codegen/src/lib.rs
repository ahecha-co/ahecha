#![feature(proc_macro_span)]

extern crate proc_macro;

use core::panic;

use nom::error::ErrorKind;
use proc_macro::TokenStream;
use proc_macro_error::proc_macro_error;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

mod api;
mod custom_element;
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

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
  // If there's a better way to stringify a TokenStream without losing the original format, please let me know.
  let input_html = input
    .to_string()
    .replace("\n", "")
    .replace("\r", "")
    .replace("\t", "")
    .replace("<! ", "<!")
    .replace("<!- -", "<!--")
    .replace("} }", "}}")
    .replace("{ {", "{{")
    .replace("- ->", "-->")
    .replace("< ", "<")
    .replace(" >", ">")
    .replace("< /", "</")
    .replace(" / >", "/>")
    .replace("> ", ">")
    .replace(" <", "<")
    .replace(" = ", "=")
    .replace("= ", "=")
    .replace(" =", "=")
    .replace("/ ", "/");

  match html::parse::<(&str, ErrorKind)>(&input_html) {
    Ok((res, parsed_html)) => {
      assert!(
        res.is_empty(),
        "Couldn't parse the following code:\n\n```\n{}\n```\n\nIf you think is a bug please report it in Github with a minimal reproducible example. https://github.com/ahecha-co/ahecha/issues",
        res
      );

      let mut tuple_list = quote! { () };

      for node in parsed_html.iter().rev() {
        tuple_list = quote! { (#node, #tuple_list) }
      }

      quote! {
        (#tuple_list)
      }
      .into()
    }
    Err(e) => panic!("{}", e),
  }
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn partial(_metadata: TokenStream, input: TokenStream) -> TokenStream {
  let f = parse_macro_input!(input as ItemFn);
  partial::create_partial(f)
}

#[proc_macro_attribute]
#[proc_macro_error]
pub fn page(_metadata: TokenStream, nput: TokenStream) -> TokenStream {
  let f = parse_macro_input!(nput as ItemFn);
  page::create_page(f)
}
