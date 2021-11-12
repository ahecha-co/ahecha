#![feature(proc_macro_span)]

extern crate proc_macro;

use core::panic;

use nom::error::ErrorKind;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

mod custom_element;
mod html;

#[proc_macro_attribute]
pub fn custom_element(_metadata: TokenStream, item: TokenStream) -> TokenStream {
  let f = parse_macro_input!(item as ItemFn);
  custom_element::create_custom_element(f)
}

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
  let input_html = input
    .to_string()
    .replace("\n", "")
    .replace("\r", "")
    .replace("\t", "")
    .replace("<! ", "<!")
    .replace("<!- -", "<!--")
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
