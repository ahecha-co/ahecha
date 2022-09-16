use std::fs::create_dir_all;

use proc_macro_error::abort_call_site;
use quote::{quote, ToTokens};
use serde::{Deserialize, Serialize};
use syn::{AttributeArgs, ItemFn};

use crate::{
  file_path_from_call_site, module_path_from_call_site, write_to_target, FnArg, Method, Route,
  TARGET_PATH,
};

struct ApiAttributes {
  absolute_path: Option<String>,
  methods: Vec<Method>,
  path_segments: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum ReturnTy {
  Json,
  Result,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ApiRoute {
  pub(crate) args: Vec<FnArg>,
  pub(crate) ident: String,
  pub(crate) methods: Vec<Method>,
  pub(crate) module_path: String,
  pub(crate) path: String,
  pub(crate) return_ty: ReturnTy,
}

impl ToTokens for ApiRoute {
  fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
    let route_path = &self.path;
    let module_path: quote::__private::TokenStream =
      format!("{}::{}", &self.module_path, &self.ident)
        .parse::<quote::__private::TokenStream>()
        .unwrap();
    quote!(
      .route(#route_path, axum::routing::get( #module_path ))
    )
    .to_tokens(tokens);
  }
}

fn parse_attributes(attr: AttributeArgs) -> ApiAttributes {
  let mut methods = vec![];
  let mut absolute_path = None;

  for meta in attr.iter() {
    match meta {
      syn::NestedMeta::Meta(meta) => match meta {
        syn::Meta::Path(path) => {
          if let Some(ident) = path.get_ident() {
            let ident_str = ident.to_string();
            match ident_str.as_str() {
              "DELETE" => methods.push(Method::Delete),
              "GET" => methods.push(Method::Get),
              "PATCh" => methods.push(Method::Patch),
              "POST" => methods.push(Method::Post),
              "PUT" => methods.push(Method::Put),
              _ => {
                dbg!(&meta);
                todo!();
              }
            }
          }
        }
        syn::Meta::List(_) => {
          dbg!(&meta);
          todo!();
        }
        syn::Meta::NameValue(_) => {
          dbg!(&meta);
          todo!();
        }
      },
      syn::NestedMeta::Lit(lit) => match lit {
        syn::Lit::Str(value) => absolute_path = Some(value.value()),
        _ => {
          dbg!(&meta);
          todo!();
        }
      },
    }
  }

  ApiAttributes {
    absolute_path,
    methods,
    path_segments: vec![],
  }
}

pub(crate) fn parse(item: ItemFn, attr: AttributeArgs) {
  let attr = parse_attributes(attr);
  create_dir_all(TARGET_PATH).unwrap();
  let ident = item.sig.ident;
  let return_ty = {
    match item.sig.output {
      syn::ReturnType::Default => {
        abort_call_site!("The function must have a return and must be either Json or Result")
      }
      syn::ReturnType::Type(_, ty) => {
        let ty = ty.to_token_stream().to_string();
        if ty.starts_with("Json") {
          ReturnTy::Json
        } else if ty.starts_with("Result") {
          ReturnTy::Result
        } else {
          abort_call_site!("Only Json and Result are supported return types");
        }
      }
    }
  };
  let file_path = file_path_from_call_site();
  let parts = file_path.split("src/").collect::<Vec<_>>();
  let file_path = parts.get(1).unwrap().trim_end_matches(".rs");
  let path = match attr.absolute_path {
    Some(path) => {
      if attr.path_segments.is_empty() {
        path
      } else {
        let mut path = path.trim_end_matches("/").split("/").collect::<Vec<_>>();
        let _ = path.pop();
        format!("{}/{}", path.join("/"), attr.path_segments.join("/"))
      }
    }
    None => file_path.trim_end_matches("index").to_owned(),
  };
  let module_path = module_path_from_call_site();
  let route = ApiRoute {
    args: vec![],
    ident: ident.to_string(),
    methods: attr.methods,
    module_path,
    path: format!("/{}", path.trim_start_matches("/").trim_end_matches("/")),
    return_ty,
  };
  write_to_target(
    "route",
    &format!("{}-{}", &route.module_path, &route.ident),
    Route::Api(route),
  );
}
