use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use syn::{
  parse::{Parse, ParseStream},
  Field, Fields, ItemStruct,
};

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

pub struct Page {
  item: ItemStruct,
  path: PagePath,
}

impl Parse for Page {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let item: ItemStruct = input.parse()?;
    let path = if let Some(route) = item.attrs.iter().find(|a| {
      let a_path = &a.path;
      quote!(#a_path).to_string() == "route"
    }) {
      let path = route.parse_args::<syn::LitStr>()?;
      PagePath::new(path.value(), &item.ident, &item.fields)
    } else {
      return Err(syn::Error::new(
        item.ident.span(),
        "`#[route]` attribute is required",
      ));
    };

    if missing_field(&item.fields, "partial") {
      return Err(syn::Error::new(
        item.ident.span(),
        "The field `partial: ahecha::html::PartialBuilder` is required",
      ));
    } else if invalid_type(&item.fields, "partial", "PartialBuilder") {
      return Err(syn::Error::new(
        item.ident.span(),
        "The field `partial` must be of type `ahecha::html::partials::PartialBuilder`",
      ));
    }

    Ok(Self { item, path })
  }
}

impl ToTokens for Page {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let path = &self.path;
    let internal_impl = internal_impl(&self.item, &self.path);

    let res = quote!(
      #internal_impl

      #path
    );
    dbg!(res.to_string());

    quote!(
      #internal_impl

      #path
    )
    .to_tokens(tokens);
  }
}

fn internal_impl(item: &ItemStruct, path: &PagePath) -> TokenStream {
  let ident = &item.ident;
  let mod_ident = Ident::new(
    format!("__internal__{}", &item.ident).as_str(),
    item.ident.span(),
  );
  let fields = &item
    .fields
    .iter()
    .filter(|f| {
      if let Some(ident) = &f.ident {
        ident.to_string() != "partial"
      } else {
        false
      }
    })
    .collect::<Vec<_>>();
  let params = &item.fields.iter().map(|f| &f.ident).collect::<Vec<_>>();
  let page_component = impl_page_component(item);
  let path_ident = &path.ident;
  let path_field = if path.fields.is_empty() {
    quote!(_: #path_ident)
  } else {
    let path_fields = &path.fields;
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
      use axum::response::IntoResponse;
      use ahecha::html::Component;
      use super::{ #ident, #path_ident };

      pub async fn handler(#path_field, ___layout: ahecha::html::partials::PartialLayout, #(#fields),*) -> axum::response::Response {
        ___layout.render(|partial| {
          #ident {
            #(#path_params),*
            #(#params),*
          }.view()
        }).into_response()
      }

      #page_component
    }
  )
}

fn impl_page_component(item: &ItemStruct) -> TokenStream {
  let ident = &item.ident;
  let generics = &item.generics;
  let generics_params = if item.generics.params.is_empty() {
    quote!()
  } else {
    let params = &item.generics.params;
    quote!(< #params >)
  };
  quote!(
    #[doc(hidden)]
    impl #generics_params ahecha_extra::PageRoute for #ident #generics {
      fn mount() -> axum::Router {
        use axum_extra::routing::RouterExt;
        axum::Router::new().typed_get(handler)
      }
    }
  )
}

fn missing_field(fields: &Fields, field: &str) -> bool {
  fields
    .iter()
    .filter(|f| {
      if let Some(f) = &f.ident {
        f.to_string() == field
      } else {
        false
      }
    })
    .collect::<Vec<_>>()
    .is_empty()
}

fn invalid_type(fields: &Fields, field: &str, ty: &str) -> bool {
  fields
    .iter()
    .filter(|f| {
      if let Some(f) = &f.ident {
        f.to_string() == field
      } else {
        false
      }
    })
    .filter(|f| {
      let f_ty = &f.ty;
      quote!(#f_ty).to_string().contains(ty)
    })
    .collect::<Vec<_>>()
    .is_empty()
}
