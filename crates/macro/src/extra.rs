use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
  parse::{Parse, ParseStream},
  Field, Fields, ItemStruct, Path,
};

enum Method {
  DELETE(Span),
  GET(Span),
  PATCH(Span),
  POST(Span),
  PUT(Span),
}

impl Method {
  pub fn from(path: &Path) -> Self {
    let ident = &path.segments.first().unwrap().ident;
    match ident.to_string().to_lowercase().as_str() {
      "delete" => Method::DELETE(ident.span()),
      "get" => Method::GET(ident.span()),
      "patch" => Method::PATCH(ident.span()),
      "post" => Method::POST(ident.span()),
      "put" => Method::PUT(ident.span()),
      _ => panic!("Unknown method"),
    }
  }
}

impl ToTokens for Method {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let method = match self {
      Method::DELETE(span) => Ident::new("DELETE", *span),
      Method::GET(span) => Ident::new("GET", *span),
      Method::PATCH(span) => Ident::new("PATH", *span),
      Method::POST(span) => Ident::new("POST", *span),
      Method::PUT(span) => Ident::new("PUT", *span),
    };
    quote!( ahecha_extra::HttpMethod:: #method ).to_tokens(tokens);
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
    let span = item.ident.span();

    Ok(Self {
      item,
      layout_ident,
      methods: if methods.is_empty() {
        vec![Method::GET(span)]
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
      Method::DELETE(_) => quote!(typed_delete),
      Method::GET(_) => quote!(typed_get),
      Method::PATCH(_) => quote!(typed_patch),
      Method::POST(_) => quote!(typed_post),
      Method::PUT(_) => quote!(typed_put),
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

pub struct PageV2 {
  is_nested: bool,
  item: ItemStruct,
  methods: Vec<Method>,
  path: PagePath,
}

impl PageV2 {
  pub fn set_nested(&mut self, is_nested: bool) {
    self.is_nested = is_nested;
  }
}

impl Parse for PageV2 {
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
      is_nested: false,
      item,
      methods,
      path,
    })
  }
}

impl ToTokens for PageV2 {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let ident = &self.item.ident;
    let generics_params = if self.item.generics.params.is_empty() {
      quote!()
    } else {
      let params = &self.item.generics.params;
      quote!(< #params >)
    };
    let page_path = &self.path;
    let generics = &self.item.generics;
    let fields_ty: Vec<TokenStream> = vec![];
    let fields: Vec<TokenStream> = vec![];
    let mut extra_fields_ty = vec![];
    let path_ident = Ident::new(format!("{}Path", ident).as_str(), ident.span());
    let path_fields = vec![quote!()];
    let methods = &self.methods;

    let render = if self.is_nested {
      quote! { (status, body).into_response() }
    } else {
      extra_fields_ty.push(quote! { layout: <#ident as Page>::Layout });
      quote! {
        use ahecha_extra::Layout;
        if scope.is_partial() {
          (status, body).into_response()
        } else {
          layout.render(page.slots().await, body).into_response()
        }
      }
    };

    quote! {
      #page_path

      impl #generics_params ahecha_extra::page::NestedPage for #ident #generics {
        type Path = #path_ident;

        fn methods() -> Vec<ahecha_extra::HttpMethod> {
          vec![#(#methods),*]
        }
      }

      impl #generics_params #ident #generics {
        pub fn mount(mut router: axum::Router) -> axum::Router {
          use axum_extra::routing::RouterExt;
          use ahecha_extra::page::Component;
          async fn handler(
            #path_ident { #(#path_fields),* }: #path_ident,
            scope: ahecha_extra::Scope,
            #(#extra_fields_ty),*
            #(#fields_ty),*
          ) -> axum::response::Response {
            let page = #ident {
              #(#path_fields),*
              #(#fields),*
            };

            let (status, body, debug) = match page.render(scope.clone()).await {
              Ok((scope, body)) => (scope.status(), body, Node::None),
              Err((status, body, debug)) => (
                status,
                body,
                if scope.is_debug() { debug } else { Node::None },
              ),
            };

            let body = html!(<>{body}{debug}</>);

            #render
          }

          for method in <#ident as ahecha_extra::page::NestedPage>::methods().iter() {
            router = match method {
              ahecha_extra::HttpMethod::DELETE => router.typed_delete(handler),
              ahecha_extra::HttpMethod::GET => router.typed_get(handler),
              ahecha_extra::HttpMethod::PATCH => router.typed_patch(handler),
              ahecha_extra::HttpMethod::POST => router.typed_post(handler),
              ahecha_extra::HttpMethod::PUT => router.typed_put(handler),
            }
          }

          router
        }
      }
    }
    .to_tokens(tokens);
  }
}
