use proc_macro2::{Ident, Span};
use proc_macro_error::emit_error;
use quote::quote;
use syn::{punctuated::Punctuated, spanned::Spanned, token::Comma, FnArg, Pat, PatIdent, PatType};

use crate::utils::FnStruct;

pub enum RouteType {
  Api,
  Page,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum HttpMethod {
  Get,
  Post,
  Put,
  Delete,
  Patch,
  Head,
  Options,
}

impl From<String> for HttpMethod {
  fn from(method: String) -> Self {
    match method.to_lowercase().as_str() {
      "post" => HttpMethod::Post,
      "put" => HttpMethod::Put,
      "delete" => HttpMethod::Delete,
      "patch" => HttpMethod::Patch,
      "head" => HttpMethod::Head,
      "options" => HttpMethod::Options,
      _ => HttpMethod::Get,
    }
  }
}

impl From<Ident> for HttpMethod {
  fn from(method: Ident) -> Self {
    match method.to_string().to_lowercase().as_str() {
      "post" => HttpMethod::Post,
      "put" => HttpMethod::Put,
      "delete" => HttpMethod::Delete,
      "patch" => HttpMethod::Patch,
      "head" => HttpMethod::Head,
      "options" => HttpMethod::Options,
      _ => HttpMethod::Get,
    }
  }
}

impl Into<Ident> for HttpMethod {
  fn into(self) -> Ident {
    match self {
      HttpMethod::Get => Ident::new("get", Span::call_site()),
      HttpMethod::Post => Ident::new("post", Span::call_site()),
      HttpMethod::Put => Ident::new("put", Span::call_site()),
      HttpMethod::Delete => Ident::new("delete", Span::call_site()),
      HttpMethod::Patch => Ident::new("patch", Span::call_site()),
      HttpMethod::Head => Ident::new("head", Span::call_site()),
      HttpMethod::Options => Ident::new("options", Span::call_site()),
    }
  }
}

#[derive(Clone)]
pub struct RoutePartDynamic {
  ident: Ident,
  ty: Box<syn::Type>,
}

impl RoutePartDynamic {
  fn from(arg: &FnArg) -> Option<Self> {
    match arg {
      FnArg::Typed(PatType { pat, ty, .. }) => {
        if let Pat::Ident(PatIdent { ident, .. }) = &**pat {
          Some(RoutePartDynamic {
            ident: ident.clone(),
            ty: ty.clone(),
          })
        } else {
          None
        }
      }
      _ => None,
    }
  }

  fn cmp(&self, ident: &str) -> bool {
    &self.ident.to_string() == ident
  }
}

impl ToString for RoutePartDynamic {
  fn to_string(&self) -> String {
    format!("<{}>", self.ident)
  }
}

#[derive(Clone)]
pub enum RoutePart {
  Static(String),
  Dynamic(RoutePartDynamic),
}

impl ToString for RoutePart {
  fn to_string(&self) -> String {
    match self {
      RoutePart::Static(s) => s.clone(),
      RoutePart::Dynamic(d) => d.to_string(),
    }
  }
}

// #[derive(Debug)]
pub struct Route {
  pub method: HttpMethod,
  pub parts: Vec<RoutePart>,
  pub route_type: RouteType,
}

impl Route {
  pub fn new(route_type: RouteType, url_path: String, fields: &Punctuated<FnArg, Comma>) -> Self {
    let mut url_params = fields.iter().flat_map(|arg| RoutePartDynamic::from(arg));
    let parts = url_path
      .split('/')
      .map(|part| {
        let mut part = if part.ends_with(".rs") {
          part.get(..part.len() - 3).unwrap().to_string()
        } else {
          part.to_string()
        };

        if part.starts_with("__") && part.ends_with("__") {
          if let Some(part) =
            url_params.find(|param| param.cmp(part.get(2..part.len() - 2).unwrap()))
          {
            RoutePart::Dynamic(part.clone())
          } else {
            emit_error!(part.span(), "route parameter `{}` not found", part);
            RoutePart::Static(part.to_string())
          }
        } else {
          part = if part == "index" {
            "".to_string()
          } else {
            part
          };

          RoutePart::Static(part)
        }
      })
      .collect::<Vec<RoutePart>>();

    Self {
      method: HttpMethod::Get,
      parts,
      route_type,
    }
  }

  pub fn build(&self, fn_struct: &FnStruct) -> proc_macro2::TokenStream {
    let block = fn_struct.block();

    quote! {
      #block
    }
  }

  pub fn build_uri(&self) -> proc_macro2::TokenStream {
    let params = self
      .parts
      .iter()
      .flat_map(|part| match part {
        RoutePart::Static(_) => None,
        RoutePart::Dynamic(d) => {
          let ident = d.ident.clone();
          Some(quote! { #ident })
        }
      })
      .collect::<Vec<_>>();

    let url_path = self
      .parts
      .iter()
      .map(|part| match part {
        RoutePart::Static(s) => s.clone(),
        RoutePart::Dynamic(_) => "{}".to_string(),
      })
      .collect::<Vec<_>>()
      .join("/");

    if params.is_empty() {
      quote! {
        #url_path .to_string()
      }
    } else {
      quote! {
        format!(#url_path, #(#params,)*)
      }
    }
  }

  pub fn params_types(&self) -> proc_macro2::TokenStream {
    let types = self
      .parts
      .iter()
      .filter_map(|part| match part {
        RoutePart::Static(_) => None,
        RoutePart::Dynamic(d) => Some(&d.ty),
      })
      .map(|ty| quote!( #ty ))
      .collect::<Vec<_>>();

    quote!( #(#types),* )
  }
}

pub fn generate_route_path(route_type: RouteType, fields: &Punctuated<FnArg, Comma>) -> Route {
  let span = proc_macro::Span::call_site();
  let source = span.source_file();
  let path = source.path().to_str().unwrap().to_owned();

  let url_path = match route_type {
    RouteType::Api => path.split("/api/"),
    RouteType::Page => path.split("/page/"),
  }
  .map(|s| s.to_string())
  .collect::<Vec<_>>()
  .get(1)
  .unwrap()
  .to_owned();

  Route::new(route_type, format!("/{}", url_path), fields)
}