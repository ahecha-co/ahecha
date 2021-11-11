#![feature(proc_macro_span)]

extern crate proc_macro;

use core::panic;

use convert_case::{Case, Casing};
use proc_macro::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{parse_macro_input, FnArg, ItemFn, ItemStruct, Pat};

use crate::{component::ComponentBuilder, route::path_route_builder, view::HtmlSource};

mod component;
mod functional_component;
mod html;
mod route;
mod view;

#[proc_macro_attribute]
pub fn component(_metadata: TokenStream, item: TokenStream) -> TokenStream {
  let f = parse_macro_input!(item as ItemFn);
  functional_component::create_functional_component(f)
}

#[proc_macro_attribute]
pub fn route(_metadata: TokenStream, input: TokenStream) -> TokenStream {
  let tokens = parse_macro_input!(input as ItemFn);
  let ident = tokens.sig.ident.to_string();
  let ident_path = format_ident!("{}_path", ident);
  let mutation_methods = vec!["post", "put", "delete", "patch"];
  let mut methods: Vec<&str> = vec!["get", "head", "connect", "options", "trace"];
  methods.extend(&mutation_methods);
  let method = methods
    .iter()
    .find(|&m| ident == *m || ident.starts_with(&format!("{}_", m)));

  if let Some(method) = method {
    let method = format_ident!("{}", method);
    let span = Span::call_site();
    let source = span.source_file();
    if let Some(route) = path_route_builder(source.path().to_str().unwrap()) {
      let path = &route.path;
      let path_with_params = &route.path_with_params;
      let mut replace = vec![];
      let fn_args = tokens
        .sig
        .inputs
        .clone()
        .into_iter()
        .filter(|i| match i {
          FnArg::Typed(pat_type) => match &*pat_type.pat {
            Pat::Ident(pat_ident) => route
              .path_params
              .contains(&pat_ident.clone().ident.to_string()),
            _ => false,
          },
          FnArg::Receiver(_) => false,
        })
        .map(|i| {
          let pat_type = match i {
            FnArg::Typed(typed) => Some(typed),
            FnArg::Receiver(_) => None,
          }
          .unwrap();
          let pat_ident = match *pat_type.pat {
            Pat::Ident(pat_ident) => Some(pat_ident),
            _ => None,
          }
          .unwrap();
          let ty = *pat_type.ty;
          replace.push(quote! { #pat_ident });
          quote! { #pat_ident: #ty }
        })
        .collect::<Vec<_>>();

      quote! {
        #[cfg(feature = "rocket")]
        #[rocket::#method(#path_with_params)]
        #tokens

        pub fn #ident_path(#(#fn_args),*) -> String {
          format!(#path, #(#replace),*)
        }
      }
      .into()
    } else {
      panic!("The file path `{:?}` isn't a valid route", source.path());
    }
  } else {
    panic!(
      "Couldn't determine the HTTP method of {}. The supported methods are {}",
      ident,
      methods.join(", ")
    )
  }
}

#[proc_macro_attribute]
pub fn page(_metadata: TokenStream, input: TokenStream) -> TokenStream {
  let tokens = parse_macro_input!(input as ItemStruct);
  let ident = &tokens.ident;
  let component_builder = ComponentBuilder::new(&tokens);
  let fields = component_builder.get_fields_declaration();
  let implementations = component_builder.implementations();

  let span = Span::call_site();
  let source = span.source_file();
  if let Some(route) = path_route_builder(source.path().to_str().unwrap()) {
    let path = &route.path;
    // let ident_path = format_ident!("{}_path", ident.to_string().to_case(Case::Snake));
    let ident_route = format_ident!("{}_route", ident.to_string().to_case(Case::Snake));
    quote! {
      use ita::view::{CustomElement, Renderable};

      // #[cfg(feature = "backend", feature="rocket")]
      #[cfg(feature="rocket")]
      #[rocket::get(#path)]
      pub fn #ident_route<'a>() -> #ident<'a> {
        #ident ::default()
      }

      pub struct #ident<'a> {
        #fields
      }

      #implementations

      // pub fn #ident_path() -> String {
      //   format!(#path)
      // }

      #[cfg(feature = "rocket")]
      impl<'a> rocket::response::Responder<'a, 'static> for #ident<'a> {
        fn respond_to(self, _: &'a rocket::request::Request<'_>) -> rocket::response::Result<'static> {
          let body = self.render().to_string();
          rocket::response::Response::build()
            .sized_body(body.len(), std::io::Cursor::new(body))
            .header(rocket::http::ContentType::new("text", "html"))
            .ok()
        }
      }
    }
    .into()
  } else {
    panic!("Couldn't generate route for {:?}", ident)
  }
}

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
  let view = parse_macro_input!(input as HtmlSource);
  quote! {
    #view .into()
  }
  .into()
}

#[proc_macro]
pub fn html_parser(input: TokenStream) -> TokenStream {
  let input_html = input
    .to_string()
    .replace("< ", "<")
    .replace(" >", ">")
    .replace("< /", "</")
    .replace(" / >", "/>")
    .replace("> ", ">")
    .replace(" <", "<")
    .replace(" = ", "=")
    .replace("/ ", "/");
  let (res, parsed_html) = html::parse(input_html.as_bytes()).unwrap();
  assert!(res.is_empty());
  quote! {
    #parsed_html
  }
  .into()
}
