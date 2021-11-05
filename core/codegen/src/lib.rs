#![feature(proc_macro_span)]

// /// The `Document` is the top root element and needs to be declared in `src/document.rs`,
// /// the pages are rendered inside a document, you can change the document title by calling
// /// `self.document.head.title` from your page
// #[proc_macro_derive(Document, attributes(props, params))]
// pub fn derive_document(_item: TokenStream) -> TokenStream {
//     TokenStream::new()
// }

// /// The `Page` has several roles:
// /// - Serves at the main entry point.
// /// - The `Page` needs to be in the `pages` folder.
// /// - The folder path starting from `pages` (root) will be used to build the URL.
// /// - You can specify the server props that will be set on each request.
// /// - A page needs to implement the Page trait.
// #[proc_macro_derive(Page, attributes(props, params))]
// pub fn derive_page(_item: TokenStream) -> TokenStream {
//     TokenStream::new()
// }

// /// The `Component` is built on top of web components, and needs to implement the `Component`
// /// trait.
// #[proc_macro_derive(Component, attributes(props, state))]
// pub fn derive_component(_item: TokenStream) -> TokenStream {
//     TokenStream::new()
// }

// /// The `Model` defines the table structure, the migrations are generated by the diff on each build
// /// if any.
// /// Also implements the methods ActiveRecord like methods.
// /// When using the `Model` derive you need to explicitly set the database connection instance that
// /// will be used.
// #[proc_macro_derive(Model, attributes())]
// pub fn derive_model(_item: TokenStream) -> TokenStream {
//     TokenStream::new()
// }

// /// The `ApiMutation` defines an API route that is intended to be used to mutate (create/update/delete) data,
// /// a mutation is required to have a data validator.
// #[proc_macro_derive(ApiMutation, attributes())]
// pub fn derive_api_mutation(_item: TokenStream) -> TokenStream {
//     TokenStream::new()
// }

// // Define `route` attribute macro

// Define `prepare_routes` macro
// This macro will get all routes from api::* and pages::* and generate the corresponding routes

extern crate proc_macro;

use core::panic;

use convert_case::{Case, Casing};
use proc_macro::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn, ItemStruct};

use crate::{component::ComponentBuilder, route::path_route_builder, view::HtmlSourceNode};

mod component;
mod route;
mod view;

#[proc_macro_attribute]
pub fn component(_metadata: TokenStream, input: TokenStream) -> TokenStream {
  let tokens = parse_macro_input!(input as ItemStruct);
  let component_builder = ComponentBuilder::new(&tokens);
  let ident = &tokens.ident;
  let fields = component_builder.get_fields_declaration();
  let implementations = component_builder.implementations();

  quote! {
    #[derive(Clone)]
    pub struct #ident {
      #fields
    }

    #implementations
  }
  .into()
}

#[proc_macro_attribute]
pub fn route(_metadata: TokenStream, input: TokenStream) -> TokenStream {
  let tokens = parse_macro_input!(input as ItemFn);
  let ident = tokens.sig.ident.to_string();
  // let ident_path = format_ident!("{}_path", ident);
  let mutation_methods = vec!["post", "put", "delete", "patch"];
  let mut methods: Vec<&str> = vec!["get", "head", "connect", "options", "trace"];
  methods.extend(&mutation_methods);
  let method = methods
    .iter()
    .find(|&m| ident == m.to_string() || ident.starts_with(&format!("{}_", m)));

  if let Some(method) = method {
    let method = format_ident!("{}", method);
    let span = Span::call_site();
    let source = span.source_file();
    if let Some(route) = path_route_builder(source.path().to_str().unwrap()) {
      let path = &route.path;
      // let mut replace = vec![];
      // let fn_args = tokens
      //   .sig
      //   .inputs
      //   .clone()
      //   .into_iter()
      //   .filter(|i| match i {
      //     FnArg::Typed(pat_type) => match &*pat_type.pat {
      //       Pat::Ident(pat_ident) => route
      //         .path_params
      //         .contains(&pat_ident.clone().ident.to_string()),
      //       _ => false,
      //     },
      //     FnArg::Receiver(_) => false,
      //   })
      //   .map(|i| {
      //     let pat_type = match i {
      //       FnArg::Typed(typed) => Some(typed),
      //       FnArg::Receiver(_) => None,
      //     }
      //     .unwrap();
      //     let pat_ident = match *pat_type.pat {
      //       Pat::Ident(pat_ident) => Some(pat_ident),
      //       _ => None,
      //     }
      //     .unwrap();
      //     let ty = *pat_type.ty;
      //     replace.push(quote! { #pat_ident });
      //     quote! { #pat_ident: #ty }
      //   })
      //   .collect::<Vec<_>>();

      quote! {
        #[cfg(feature = "rocket")]
        use rocket::#method;
        #[cfg(feature = "rocket")]
        #[#method(#path)]
        #tokens

        // pub fn #ident_path(#(#fn_args),*) -> String {
        //   format!(#path, #(#replace),*)
        // }
      }
      .into()
    } else {
      panic!(
        "The file path `{:?}` isn't a valid route",
        source.path().to_owned()
      );
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
      use etagere::view::{CustomElement, Renderable};

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

// #[proc_macro_attribute]
// pub fn etagere_init(_metadata: TokenStream, input: TokenStream) -> TokenStream {
//   let tokens = parse_macro_input!(input as ItemFn);
//   let out_dir = env::var_os("OUT_DIR").unwrap();
//   let dest_path = Path::new(&out_dir).join("routes.rs");
//   let dest_path_str = dest_path.to_str();
//   quote! {
//     // #[macro_use] extern crate etagere::rocket;

//     include!(#dest_path_str);

//     #tokens
//   }
//   .into()
// }

#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
  let view = parse_macro_input!(input as HtmlSourceNode);
  quote! { #view }.into()
}
