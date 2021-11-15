use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use proc_macro_error::emit_error;
use quote::quote;
use syn::spanned::Spanned;

use crate::utils::FnStruct;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Route {
  pub method: HttpMethod,
  pub name: String,
  pub parts: Vec<String>,
  pub route_type: RouteType,
}

impl Route {
  fn new(name: String, route_type: RouteType, url_path: String) -> Self {
    let mut parts: Vec<String> = if url_path.ends_with(".rs") {
      url_path[..url_path.len() - 3].split('/')
    } else {
      url_path.split('/')
    }
    .into_iter()
    .filter(|s| !s.is_empty())
    .map(|s| {
      let s = s.to_string();

      if s.starts_with("__") && s.ends_with("__") {
        format!("<{}>", &s[2..s.len() - 2])
      } else {
        s
      }
    })
    .collect();

    let mut last: String = parts.clone().into_iter().last().unwrap();
    last = if last.starts_with("index") {
      last[5..].to_string()
    } else {
      last
    };

    let last_index = parts.len() - 1;
    let _ = std::mem::replace(&mut parts[last_index], last);

    Self {
      method: HttpMethod::Get,
      name,
      parts: parts
        .iter()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_owned())
        .collect::<Vec<_>>()
        .to_owned(),
      route_type,
    }
  }

  fn path_name(&self) -> String {
    format!("/{}", self.parts.join("/"))
  }

  fn build(&self, fn_struct: &FnStruct) -> proc_macro2::TokenStream {
    let method: Ident = self.method.into();
    // let mut route = quote! { warp:: #method () };
    let mut route: Option<proc_macro2::TokenStream> = None;

    for part in self.parts.iter() {
      // let value = if part.starts_with("<") && part.ends_with(">") {
      //   let part = part[1..part.len() - 1].to_string();
      //   let part = Ident::new(&part, Span::call_site());
      //   quote!( warp::path( #part ))
      // } else {
      if let Some(r) = route {
        route = Some(quote!( #r . #method ( #part ) ));
      } else {
        route = Some(quote!( warp::path( #part ) ));
      }
      // };
    }

    let block = fn_struct.block();
    let input_readings = if fn_struct.inputs().is_empty() {
      quote!()
    } else {
      let input_names: Vec<_> = fn_struct
        .inputs()
        .iter()
        .filter_map(|argument| match argument {
          syn::FnArg::Typed(typed) => Some(typed),
          syn::FnArg::Receiver(rec) => {
            emit_error!(rec.span(), "Don't use `self` on components");
            None
          }
        })
        .map(|value| {
          let pat = &value.pat;
          quote!(#pat)
        })
        .collect();

      quote!(
        #(#input_names),*,
      )
    };

    let route = if let Some(route) = route {
      quote! { #route .and(warp::path::end()) }
    } else {
      quote! { warp::path::end() }
    };

    quote! {
      use warp::Filter;

      #route .map(|#input_readings| {
          warp::http::Response::builder()
            .header("Content-Type", "text/html")
            .body( #block .render() )
        })
    }
  }
}

pub fn generate_route_path(name: String, route_type: RouteType) -> Route {
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

  Route::new(name, route_type, format!("/{}", url_path))
}

pub fn create_page(f: syn::ItemFn) -> TokenStream {
  let fn_struct: FnStruct = f.into();

  let vis = fn_struct.vis();
  let struct_name = fn_struct.name();
  let impl_generics = fn_struct.impl_generics();
  let ty_generics = fn_struct.type_generics();
  let where_clause = fn_struct.where_clause();
  let input_blocks = fn_struct.input_blocks();

  let struct_str_name = struct_name.to_string();
  if struct_str_name.to_uppercase().chars().next().unwrap()
    != struct_str_name.chars().next().unwrap()
  {
    emit_error!(struct_name.span(), "Pages must start with a upper letter");
  }

  if !struct_str_name.ends_with("Page") {
    emit_error!(
      struct_name.span(),
      "Pages must have the `Page` suffix, example: `{}Page`",
      struct_str_name
    );
  }

  let route = generate_route_path(struct_name.to_string(), RouteType::Page);
  let path_name = route.path_name();
  let mount_route = route.build(&fn_struct);

  quote! {
    #[derive(Debug)]
    #vis struct #struct_name #impl_generics #input_blocks

    impl #ty_generics #struct_name #impl_generics #where_clause {
      pub fn route() -> &'static str {
        #path_name
      }

      // pub fn mount(f: Fn(impl warp::Filter<Extract=(), Error=warp::Rejection> + Clone + Send + Sync) -> (impl warp::Filter<Extract=(), Error=warp::Rejection> + Clone + Send + Sync)) {
      //   #mount_route
      // }
      pub fn mount() -> impl warp::Filter<Extract = (Result<warp::http::Response<String>, warp::http::Error>,), Error = warp::Rejection> + Copy {
        #mount_route
      }
    }

    // impl #impl_generics Into<String> for #struct_name #ty_generics #where_clause {
    //   fn into(self) -> String {
    //     self.render()
    //   }
    // }
  }
  .into()
}
