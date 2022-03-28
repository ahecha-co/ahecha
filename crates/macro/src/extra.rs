use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{
  parse::{Parse, ParseStream},
  Field, Fields, ItemStruct, Path,
};

enum Method {
  DELETE,
  GET,
  PATCH,
  POST,
  PUT,
}

impl Method {
  pub fn from(path: &Path) -> Self {
    match path
      .segments
      .first()
      .unwrap()
      .ident
      .to_string()
      .to_lowercase()
      .as_str()
    {
      "delete" => Method::DELETE,
      "get" => Method::GET,
      "patch" => Method::PATCH,
      "post" => Method::POST,
      "put" => Method::PUT,
      _ => panic!("Unknown method"),
    }
  }
}

struct PagePath {
  fields: Vec<Field>,
  ident: Ident,
  path: String,
}

impl PagePath {
  fn new(path: String, ident: &Ident, fields: &Fields) -> Self {
    let path_parts = path.clone();
    let parts = path_parts
      .split('/')
      .filter(|p| p.starts_with(":"))
      .collect::<Vec<_>>();
    Self {
      path,
      fields: fields
        .iter()
        .filter(|f| {
          if let Some(ident) = &f.ident {
            parts.contains(&format!(":{}", ident).as_str())
          } else {
            false
          }
        })
        .map(|f| f.clone())
        .collect::<Vec<_>>(),
      ident: Ident::new(format!("{}Path", ident).as_str(), ident.span()),
    }
  }
}

impl ToTokens for PagePath {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let path = &self.path;
    let ident = &self.ident;
    let fields = &self.fields;

    if fields.is_empty() {
      quote!(
        #[derive(axum_extra::routing::TypedPath, serde::Deserialize)]
        #[typed_path(#path)]
        pub struct #ident;
      )
      .to_tokens(tokens);
    } else {
      quote!(
        #[derive(axum_extra::routing::TypedPath, serde::Deserialize)]
        #[typed_path(#path)]
        pub struct #ident {
          #(#fields),*
        }
      )
      .to_tokens(tokens);
    }
  }
}

pub struct Page {
  item: ItemStruct,
  layout_ident: Ident,
  methods: Vec<Method>,
  path: PagePath,
}

impl Parse for Page {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let item: ItemStruct = input.parse()?;
    let path = if let Some(attr) = item.attrs.iter().find(|a| {
      let a_path = &a.path;
      quote!(#a_path).to_string() == "route"
    }) {
      let path = attr.parse_args::<syn::LitStr>()?;
      PagePath::new(path.value(), &item.ident, &item.fields)
    } else {
      return Err(syn::Error::new(
        item.ident.span(),
        "`#[route]` attribute is required",
      ));
    };
    let layout_ident = if let Some(attr) = item.attrs.iter().find(|a| {
      let a_path = &a.path;
      quote!(#a_path).to_string() == "layout"
    }) {
      attr.parse_args::<Ident>()?
    } else {
      return Err(syn::Error::new(
        item.ident.span(),
        "`#[layout]` attribute is required",
      ));
    };
    let methods = if let Some(attr) = item.attrs.iter().find(|a| {
      let a_path = &a.path;
      quote!(#a_path).to_string() == "method"
    }) {
      let meta = attr.parse_meta()?;

      match meta {
        syn::Meta::Path(value) => vec![Method::from(&value)],
        syn::Meta::List(value) => value
          .nested
          .iter()
          .map(|value| match value {
            syn::NestedMeta::Meta(syn::Meta::Path(value)) => Method::from(&value),
            _ => {
              panic!("Unknown method")
              // return Err(syn::Error::new(
              //   item.ident.span(),
              //   "`#[method]` attribute doesn't support the specified syntax",
              // ))
            }
          })
          .collect::<Vec<_>>(),
        _ => {
          return Err(syn::Error::new(
            item.ident.span(),
            "`#[method]` attribute doesn't support the specified syntax",
          ))
        }
      }
    } else {
      vec![]
    };

    Ok(Self {
      item,
      layout_ident,
      methods: if methods.is_empty() {
        vec![Method::GET]
      } else {
        methods
      },
      path,
    })
  }
}

impl ToTokens for Page {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let path = &self.path;
    let internal_impl = internal_impl(&self.item, &self);

    quote!(
      #internal_impl

      #path
    )
    .to_tokens(tokens);
  }
}

fn internal_impl(item: &ItemStruct, page: &Page) -> TokenStream {
  let ident = &item.ident;
  let path = &page.path;
  let layout_ident = &page.layout_ident;
  let mod_ident = Ident::new(
    format!("__internal__{}", &item.ident).as_str(),
    item.ident.span(),
  );
  let fields = &item
    .fields
    .iter()
    .filter(|f| {
      if let Some(ident) = &f.ident {
        path
          .fields
          .iter()
          .find(|f| {
            f.ident
              .as_ref()
              .map_or_else(|| false, |f_ident| f_ident == ident)
          })
          .is_none()
      } else {
        false
      }
    })
    .collect::<Vec<_>>();
  let params = fields.iter().map(|f| &f.ident).collect::<Vec<_>>();
  let page_component = impl_page_component(item, &page);
  let path_ident = &path.ident;
  let path_field = if path.fields.is_empty() {
    quote!(_: #path_ident)
  } else {
    let path_fields = &path.fields.iter().map(|f| &f.ident).collect::<Vec<_>>();
    quote!(
      #path_ident {
        #(#path_fields),*
      } : #path_ident
    )
  };
  let path_params = &path.fields.iter().map(|f| &f.ident).collect::<Vec<_>>();

  quote!(
    #[doc(hidden)]
    #[allow(non_snake_case)]
    mod #mod_ident {
      use super::*;
      use super::{ #ident, #path_ident };
      use axum::extract::FromRequest;

      pub async fn handler(#path_field, mut ___layout: ahecha_extra::view::View<#layout_ident>, #(#fields),*) -> axum::response::Response {
        use axum::response::IntoResponse;
        use ahecha_extra::Component;

        ___layout.render(#ident {
          #(#path_params,)*
          #(#params,)*
        }).await.into_response()
      }

      #page_component
    }
  )
}

fn impl_page_component(item: &ItemStruct, page: &Page) -> TokenStream {
  let ident = &item.ident;
  let generics = &item.generics;
  let generics_params = if item.generics.params.is_empty() {
    quote!()
  } else {
    let params = &item.generics.params;
    quote!(< #params >)
  };

  let typed_method = page
    .methods
    .iter()
    .map(|m| match m {
      Method::DELETE => quote!(typed_delete),
      Method::GET => quote!(typed_get),
      Method::PATCH => quote!(typed_patch),
      Method::POST => quote!(typed_post),
      Method::PUT => quote!(typed_put),
    })
    .collect::<Vec<_>>();

  quote!(
    #[doc(hidden)]
    impl #generics_params #ident #generics {
      pub fn mount() -> axum::Router {
        use axum_extra::routing::RouterExt;
        axum::Router::new() #(. #typed_method (handler))*
      }
    }
  )
}
