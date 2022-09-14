#![feature(proc_macro_span)]
use std::fs::{create_dir_all, read_dir, read_to_string, write};

use proc_macro::{Span, TokenStream};
use quote::quote;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum RouteType {
  Api,
  Page,
}

#[derive(Serialize, Deserialize, Debug)]
struct Route {
  module_path: String,
  path: String,
  ty: RouteType,
}

#[proc_macro_attribute]
pub fn page(_attr: TokenStream, item: TokenStream) -> TokenStream {
  parse_route(item.clone(), RouteType::Page);
  item
}

#[proc_macro_attribute]
pub fn route(_attr: TokenStream, item: TokenStream) -> TokenStream {
  parse_route(item.clone(), RouteType::Api);
  item
}

#[proc_macro]
pub fn router(_item: TokenStream) -> TokenStream {
  let target_path = "target/router";
  create_dir_all(target_path).unwrap();
  let dir = read_dir(target_path).unwrap();
  let mut routes = vec![];

  for path in dir {
    let path = path.unwrap();
    let path_str = path.file_name().into_string().unwrap();
    if path_str.ends_with(".json") {
      let content = read_to_string(&path.path()).unwrap();
      let route: Route = serde_json::from_str(&content).unwrap();
      let route_path = &route.path;
      let module_path = {
        let tokens = route.module_path.parse::<TokenStream>().unwrap();
        syn::parse_macro_input!(tokens as syn::Path)
      };
      match route.ty {
        RouteType::Api => routes.push(quote!(
          .route(#route_path, axum::routing::get( #module_path ))
        )),
        RouteType::Page => routes.push(quote!(
          .route(#route_path, axum::routing::get(|| async move {
            let mut vdom = dioxus::prelude::VirtualDom::new(#module_path);
            let _ = vdom.rebuild();
            axum::response::Html(
              dioxus::ssr::render_vdom(&vdom)
            )
          }))
        )),
      }
    }
  }

  quote!(
    server(
      axum::Router::new() #(#routes)*
    ).await
  )
  .into()
}

fn parse_route(item: TokenStream, ty: RouteType) {
  create_dir_all("target/router/").unwrap();
  let ident = item
    .clone()
    .into_iter()
    .find(|t| match t {
      proc_macro::TokenTree::Ident(ident) => {
        !["async", "fn", "pub"].contains(&ident.to_string().as_str())
      }
      _ => false,
    })
    .map(|t| match t {
      proc_macro::TokenTree::Ident(ident) => ident.to_string(),
      _ => unreachable!(),
    })
    .unwrap();
  let span = Span::call_site();
  let file_path = span.source_file().path().display().to_string();
  let parts = file_path.split("src/").collect::<Vec<_>>();
  let path = parts.get(1).unwrap().trim_end_matches(".rs");
  let module_path = path.replace('/', "::");
  let name = format!("route-{}.json", hash_string(&module_path));
  let route = Route {
    module_path: format!("crate::{}::{}", module_path, ident),
    path: format!("/{}", path.trim_end_matches("/")),
    ty,
  };
  dbg!(&route);
  write(
    format!("target/router/{}", name),
    serde_json::to_string_pretty(&route).unwrap(),
  )
  .unwrap();
}

fn hash_string(input: &str) -> String {
  use sha2::{Digest, Sha256};
  hex::encode(Sha256::digest(input.as_bytes()))
}
