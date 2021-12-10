use proc_macro2::{Ident, Span};
use proc_macro_error::emit_error;
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, spanned::Spanned, token::Comma, FnArg, Pat, PatIdent, PatType};

// use crate::utils::FnStruct;

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
      "get" => HttpMethod::Get,
      "post" => HttpMethod::Post,
      "put" => HttpMethod::Put,
      "delete" => HttpMethod::Delete,
      "patch" => HttpMethod::Patch,
      "head" => HttpMethod::Head,
      "options" => HttpMethod::Options,
      _ => {
        emit_error!(Span::call_site(), "Unsupported HTTP method: {}", method);
        HttpMethod::Get
      }
    }
  }
}

impl From<Ident> for HttpMethod {
  fn from(method: Ident) -> Self {
    match method.to_string().to_lowercase().as_str() {
      "get" => HttpMethod::Get,
      "post" => HttpMethod::Post,
      "put" => HttpMethod::Put,
      "delete" => HttpMethod::Delete,
      "patch" => HttpMethod::Patch,
      "head" => HttpMethod::Head,
      "options" => HttpMethod::Options,
      _ => {
        emit_error!(Span::call_site(), "Unsupported HTTP method: {}", method);
        HttpMethod::Get
      }
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
    self.ident == ident
  }
}

impl ToString for RoutePartDynamic {
  fn to_string(&self) -> String {
    format!("<{}>", self.ident)
  }
}

impl ToTokens for RoutePartDynamic {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let ident = &self.ident;
    let ty = &self.ty;
    quote!( #ident: #ty ).to_tokens(tokens);
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
}

impl Route {
  pub fn new(method: HttpMethod, url_path: String, fields: &Punctuated<FnArg, Comma>) -> Self {
    let mut url_params = fields.iter().flat_map(RoutePartDynamic::from);
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
            RoutePart::Dynamic(part)
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

    Self { method, parts }
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

  pub fn params(&self) -> proc_macro2::TokenStream {
    let types = self
      .parts
      .iter()
      .filter_map(|part| match part {
        RoutePart::Static(_) => None,
        RoutePart::Dynamic(d) => Some(d),
      })
      .map(|d| quote!( #d ))
      .collect::<Vec<_>>();

    quote!( #(#types),* )
  }

  // pub fn params_types(&self) -> proc_macro2::TokenStream {
  //   let types = self
  //     .parts
  //     .iter()
  //     .filter_map(|part| match part {
  //       RoutePart::Static(_) => None,
  //       RoutePart::Dynamic(d) => Some(&d.ty),
  //     })
  //     .map(|ty| quote!( #ty ))
  //     .collect::<Vec<_>>();

  //   quote!( #(#types),* )
  // }
}

pub fn generate_route_path(
  route_type: RouteType,
  fn_name: String,
  fields: &Punctuated<FnArg, Comma>,
) -> Route {
  let span = proc_macro::Span::call_site();
  // Note: using source_file raises an error with rust analyzer. Ref: https://github.com/rust-analyzer/rust-analyzer/issues/10710#issuecomment-962559112
  let source = span.source_file();
  let path = source.path().to_str().unwrap().to_owned();

  let url_path = match route_type {
    RouteType::Api => {
      if !path.contains("/api/") {
        emit_error!(span, "API endpoints must be in a folder called `api`");
      }
      path.split("/api/")
    }
    RouteType::Page => {
      if !path.contains("/pages/") {
        emit_error!(span, "Pages must be in a folder called `pages`");
      }
      path.split("/pages/")
    }
  }
  .map(|s| s.to_string())
  .collect::<Vec<_>>()
  .get(1)
  .unwrap()
  .to_owned();

  let method = match route_type {
    RouteType::Api => HttpMethod::from(fn_name),
    RouteType::Page => HttpMethod::Get,
  };

  Route::new(method, format!("/{}", url_path), fields)
}
