use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use proc_macro_error::emit_error;
use quote::quote;
use syn::{punctuated::Punctuated, spanned::Spanned, token::Comma, FnArg, Pat, PatPath};

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

// #[derive(Debug)]
pub struct Route {
  pub method: HttpMethod,
  pub name: String,
  pub parts: Vec<String>,
  pub route_params: Punctuated<FnArg, Comma>,
  pub route_type: RouteType,
}

impl Route {
  fn new(
    name: String,
    route_type: RouteType,
    url_path: String,
    fields: &Punctuated<FnArg, Comma>,
  ) -> Self {
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
        let field_name = &s[2..s.len() - 2];
        format!("<{}>", field_name)
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
      route_params: fields.clone(),
      route_type,
    }
  }

  fn build_uri(&self) -> proc_macro2::TokenStream {
    if self.route_params.is_empty() {
      let url_path = format!("/{}", self.parts.join("/"));
      quote! {
        #url_path .to_string()
      }
    } else {
      let params: Vec<_> = self
        .route_params
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

      let url_path = format!(
        "/{}",
        self
          .parts
          .iter()
          .map(|f| {
            if f.starts_with("<") && f.ends_with(">") {
              "{}"
            } else {
              &f
            }
          })
          .collect::<Vec<&str>>()
          .join("/")
      );

      quote! {
        format!(#url_path, #(#params,)*)
      }
    }
  }

  fn build(&self, fn_struct: &FnStruct) -> proc_macro2::TokenStream {
    let method: Ident = self.method.into();
    // let mut route = quote! { warp:: #method () };
    let mut route: Option<proc_macro2::TokenStream> = None;

    for part in self.parts.iter() {
      let uri = if part.starts_with("<") && part.ends_with(">") {
        quote!(warp::path::param())
      } else {
        quote!( $part )
      };
      if let Some(r) = route {
        route = Some(quote!( #r . #method ( #uri ) ));
      } else {
        route = Some(quote!( warp::path( #uri ) ));
      }
      // };
    }

    let block = fn_struct.block();
    let input_readings = if fn_struct.inputs().is_empty() {
      quote!()
    } else {
      let input_names: Vec<_> = fn_struct.inputs().iter().collect();
      quote!(#(#input_names),*,)
    };

    let route = if let Some(route) = route {
      quote! { #route .and(warp::path::end()) }
    } else {
      quote! { warp::path::end() }
    };

    quote! {
      {
        use warp::Filter;

        #route
          // .with(f)
          .map(|#input_readings| {
            // warp::http::Response::builder()
            //   .header("Content-Type", "text/html")
            //   .body( #block .render() )
            #block .render()
          })
      }
    }
  }
}

pub fn generate_route_path(
  name: String,
  route_type: RouteType,
  fields: &Punctuated<FnArg, Comma>,
) -> Route {
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

  Route::new(name, route_type, format!("/{}", url_path), fields)
}

pub fn create_page(f: syn::ItemFn) -> TokenStream {
  let fn_struct: FnStruct = f.into();

  let vis = fn_struct.vis();
  let struct_name = fn_struct.name();
  let impl_generics = fn_struct.impl_generics();
  let ty_generics = fn_struct.type_generics();
  let where_clause = fn_struct.where_clause();
  let input_blocks = fn_struct.input_blocks();
  let input_fields = fn_struct.input_fields();

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

  let route = generate_route_path(struct_name.to_string(), RouteType::Page, fn_struct.inputs());
  let uri = route.build_uri();
  let mount_route = route.build(&fn_struct);

  quote! {
    #[derive(Debug)]
    #vis struct #struct_name #impl_generics #input_blocks

    impl #ty_generics #struct_name #impl_generics #where_clause {
      pub fn uri(#input_fields) -> String {
        #uri
      }

      pub fn mount() -> impl warp::Filter<Extract = (String,), Error = warp::Rejection> + Clone + Send + Sync + 'static
      {
        #mount_route
      }

      // TODO: It seems that this will be easier to implement with a macro
      // pub fn mount_with_middleware<F, T>(f: F) -> impl warp::Filter<Extract = (T,)> + Clone + Send + Sync + 'static
      // where
      //   F: warp::Filter<Extract = (T,)> + Clone + Send + Sync + 'static,
      //   F::Extract: warp::Reply,
      //   // C: Fn(F) -> F,
      //   // // A: warp::Filter<Extract = (), Error = warp::Rejection> + Copy,
      //   // F: warp::Filter<Extract = (T,), Error = warp::Rejection> + Clone + Send + Sync + 'static,
      //   // F::Extract: warp::Reply,
      //   //A: warp::Filter + Clone,
      // {
      //   #mount_route
      // }
    }
  }
  .into()
}
