#![feature(proc_macro_span)]
use std::fs::{create_dir_all, read_dir, read_to_string, remove_dir_all, write};

use api::ApiRoute;
use page::{DynamicPageRoute, StaticPageRoute};
use proc_macro::{Span, TokenStream};
use proc_macro_error::proc_macro_error;
use quote::{quote, ToTokens};
use serde::{Deserialize, Serialize};
use syn::{parse_macro_input, AttributeArgs, Ident, ItemFn};

mod api;
mod page;

const TARGET_PATH: &'static str = "target/ahecha";

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum RenderStrategy {
  CSR,
  SSR,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
enum Method {
  Delete,
  Get,
  Patch,
  Post,
  Put,
}

#[derive(Serialize, Deserialize, Debug)]
struct FnArg {
  ident: String,
  ty: String,
}

impl ToTokens for FnArg {
  fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
    let ident = Ident::new(&self.ident, Span::call_site().into());
    let ty = self
      .ty
      .clone()
      .parse::<quote::__private::TokenStream>()
      .unwrap();
    quote!(#ident: #ty).to_tokens(tokens);
  }
}

#[derive(Serialize, Deserialize, Debug)]
enum Route {
  Api(ApiRoute),
  DynamicPage(DynamicPageRoute),
  StaticPage(StaticPageRoute),
}

impl ToTokens for Route {
  fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
    match self {
      Route::Api(t) => quote!(#t).to_tokens(tokens),
      Route::DynamicPage(t) => quote!(#t).to_tokens(tokens),
      Route::StaticPage(t) => quote!(#t).to_tokens(tokens),
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
struct Layout {
  ident: String,
  module_path: String,
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn layout(_attr: TokenStream, item: TokenStream) -> TokenStream {
  {
    let item = item.clone();
    let item_fn = parse_macro_input!(item as ItemFn);
    let layout = Layout {
      ident: item_fn.sig.ident.to_string(),
      module_path: module_path_from_call_site(),
    };
    write_to_target("layout", &layout.module_path.clone(), layout);
  }
  item
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn page(attr: TokenStream, item: TokenStream) -> TokenStream {
  page::parse(
    {
      let item = item.clone();
      parse_macro_input!(item as ItemFn)
    },
    parse_macro_input!(attr as AttributeArgs),
  );
  item
}

#[proc_macro_error]
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
  api::parse(
    {
      let item = item.clone();
      parse_macro_input!(item as ItemFn)
    },
    parse_macro_input!(attr as AttributeArgs),
  );
  item
}

#[proc_macro_error]
#[proc_macro]
pub fn monkey_path_clean(item: TokenStream) -> TokenStream {
  let _ = remove_dir_all(TARGET_PATH);
  let _ = create_dir_all(TARGET_PATH);
  item
}

#[proc_macro_error]
#[proc_macro]
pub fn router(_item: TokenStream) -> TokenStream {
  create_dir_all(TARGET_PATH).unwrap();
  let dir = read_dir(TARGET_PATH).unwrap();
  let mut routes = vec![];

  for path in dir {
    let path = path.unwrap();
    let path_str = path.file_name().into_string().unwrap();
    if path_str.ends_with(".json") {
      let content = read_to_string(&path.path()).unwrap();
      if path_str.starts_with("route-") {
        let route: Route = serde_json::from_str(&content).unwrap();
        routes.push(route);
      }
    }
  }

  let mut tokens = vec![];

  for route in routes.iter() {
    tokens.push(quote!(#route));
  }

  let tokens = quote!(
    server(
      axum::Router::new() #(#tokens)*
    ).await
  );

  write("../../_ahecha_debug.rs", &tokens.to_string()).unwrap();

  tokens.into()
}

fn hash_string(input: &str) -> String {
  use sha2::{Digest, Sha256};
  hex::encode(Sha256::digest(input.as_bytes()))
}

fn write_to_target<C>(name: &str, string_to_hash: &str, content: C)
where
  C: Serialize + std::fmt::Debug,
{
  write(
    format!(
      "{}/{}-{}.json",
      TARGET_PATH,
      name,
      hash_string(string_to_hash)
    ),
    serde_json::to_string_pretty(&content).unwrap(),
  )
  .unwrap();
}

fn file_path_from_call_site() -> String {
  let span = Span::call_site();
  span.source_file().path().display().to_string()
}

fn module_path_from_call_site() -> String {
  let file_path = file_path_from_call_site();
  let parts = file_path.split("src/").collect::<Vec<_>>();
  let file_path = parts.get(1).unwrap().trim_end_matches(".rs");
  format!("crate::{}", file_path.replace('/', "::"))
}

fn base_module_path(module: &str) -> String {
  let mut parts = module.split("::").collect::<Vec<_>>();
  let _ = parts.remove(parts.len() - 1);
  parts.join("::")
}
