mod component;

use std::fs::{create_dir_all, read_dir, read_to_string};

use proc_macro2::TokenStream;
use proc_macro_error::{abort_call_site, emit_error};
use quote::{quote, ToTokens, __private::Span};
use serde::{Deserialize, Serialize};
use syn::{AttributeArgs, Ident, ItemFn};

use self::component::Component;
use crate::{
  api::ApiRoute, file_path_from_call_site, module_path_from_call_site, write_to_target, FnArg,
  Method, RenderStrategy, Route, TARGET_PATH,
};

struct PageAttributes {
  absolute_path: Option<String>,
  path_segments: Vec<String>,
  render_strategy: Vec<RenderStrategy>,
  server_props: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct DynamicPageRoute {
  pub(crate) ident: String,
  pub(crate) module_path: String,
  pub(crate) path: String,
  pub(crate) props: Vec<FnArg>,
  pub(crate) render_strategy: Vec<RenderStrategy>,
  pub(crate) server_props: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct StaticPageRoute {
  pub(crate) ident: String,
  pub(crate) module_path: String,
  pub(crate) path: String,
  pub(crate) render_strategy: Vec<RenderStrategy>,
}

impl ToTokens for DynamicPageRoute {
  fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
    let route_path = &self.path;

    let api_route = match get_api_route_for(&self.server_props) {
      Some(value) => value,
      None => abort_call_site!("The route `{}` was not found.`", &self.server_props),
    };

    let handler_args = {
      let args = &api_route.args;
      quote!( #(#args),* )
    };
    let handler_ident_args = {
      let args = api_route
        .args
        .iter()
        .map(|a| a.ident.parse::<TokenStream>().unwrap())
        .collect::<Vec<_>>();
      quote!( #(#args),* )
    };

    let api_module_path = format!("{}::{}", api_route.module_path, api_route.ident)
      .parse::<quote::__private::TokenStream>()
      .unwrap();

    let props_fields = self.props.iter().map(|p| quote!( #p )).collect::<Vec<_>>();

    let props_idents = self
      .props
      .iter()
      .map(|p| {
        let ident = Ident::new(p.ident.as_str(), Span::call_site());
        quote!( #ident )
      })
      .collect::<Vec<_>>();

    let vdom_init = match api_route.return_ty {
      crate::api::ReturnTy::Json => quote!(
        let mut vdom = VirtualDom::new_with_props(app, AppProps {
          #(#props_idents: res.0. #props_idents),*
        });
      ),
      crate::api::ReturnTy::Result => quote!(
        let mut vdom = match res {
          Ok(res) => {
            VirtualDom::new_with_props(app, AppProps {
              #(#props_idents: res. #props_idents),*
            });
          },
          Err(err) => {
            #[derive(Props, PartialEq)]
            struct ErrorProps {
              #(#props_fields,)*
            }

            fn error(cx: Scope<AppProps>) -> Element {
              cx.render(rsx!(
                div {
                  class: "text-red-500 border-red-500 bg-red-200 p-8"
                  "{cx.props.error}"
                }
              ))
            }

            VirtualDom::new_with_props(error, ErrorProps {
              error: err.to_string(),
            });
          }
        }
      ),
      crate::api::ReturnTy::Redirect => {
        abort_call_site!(
          "Only `Result` and `Json` return types are supported at the moment. But the path `{}` has an unsuported return types for the page.",
          &api_route.path
        )
      }
    };

    let component = Component::build_recursive_up(self.into());
    let use_tokens = component.use_tokens();

    quote!(
      .route(#route_path, axum::routing::get(| #handler_args | async move {
        use dioxus::prelude::*;
        let index_html = include_str!("../index.html");

        #[derive(Props, PartialEq)]
        struct AppProps {
          #(#props_fields,)*
        }

        fn app(cx: Scope<AppProps>) -> Element {
          let AppProps { #(#props_idents),* } = &cx.props;
          #use_tokens
          cx.render(rsx!(
            #component
          ))
        }

        let res = #api_module_path ( #handler_ident_args ).await;
        #vdom_init

        let _ = vdom.rebuild();
        axum::response::Html(
          index_html.replace(r#"<div id="main"></div>"#, &dioxus::ssr::render_vdom(&vdom))
        )
      }))
    )
    .to_tokens(tokens);
  }
}

impl ToTokens for StaticPageRoute {
  fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
    let route_path = &self.path;
    let component = Component::build_recursive_up(self.into());
    let use_tokens = component.use_tokens();

    quote!(
      .route(#route_path, axum::routing::get(|| async move {
        use dioxus::prelude::*;
        let index_html = include_str!("../index.html");

        fn app(cx: Scope) -> Element {
          #use_tokens
          cx.render(rsx!(
            #component
          ))
        }
        let mut vdom = VirtualDom::new(app);

        let _ = vdom.rebuild();
        axum::response::Html(
          index_html.replace(r#"<div id="main"></div>"#, &dioxus::ssr::render_vdom(&vdom))
        )
      }))
    )
    .to_tokens(tokens);
  }
}

fn get_api_route_for(url_path: &str) -> Option<ApiRoute> {
  let dir = read_dir(TARGET_PATH).unwrap();

  for path in dir {
    let path = path.unwrap();
    let path_str = path.file_name().into_string().unwrap();
    if path_str.ends_with(".json") {
      let content = read_to_string(&path.path()).unwrap();
      if path_str.starts_with("route") {
        let value: Route = serde_json::from_str(&content).unwrap();
        match value {
          Route::Api(value) => {
            if &value.path == url_path {
              return Some(value);
            }
          }
          _ => (),
        }
      }
    }
  }

  None
}

fn parse_attributes(attr: AttributeArgs) -> PageAttributes {
  let mut absolute_path = None;
  let mut path_segments = vec![];
  let mut server_props = None;

  for meta in attr.iter() {
    match meta {
      syn::NestedMeta::Meta(meta) => match meta {
        syn::Meta::Path(_) => {
          dbg!(&meta);
          todo!();
        }
        syn::Meta::List(_) => {
          dbg!(&meta);
          todo!();
        }
        syn::Meta::NameValue(named) => {
          if let Some(ident) = named.path.get_ident() {
            let ident_str = ident.to_string();
            if ident_str.as_str() == "server_props" {
              match &named.lit {
                syn::Lit::Str(value) => {
                  let path = value.value();
                  match get_api_route_for(&path) {
                    Some(r) => {
                      if !r.methods.contains(&Method::Get) {
                        emit_error!(
                          value.span(),
                          "The specified api route does not support the GET method"
                        );
                      }
                    }
                    None => emit_error!(value.span(), "Api route not found"),
                  }
                  server_props = Some(path)
                }
                _ => {
                  dbg!(&meta);
                  todo!();
                }
              }
            } else {
              dbg!(&meta);
              todo!();
            }
          } else {
            dbg!(&meta);
            todo!();
          }
        }
      },
      syn::NestedMeta::Lit(lit) => match lit {
        syn::Lit::Str(value) => {
          if value.value().starts_with("~/") {
            path_segments = value
              .value()
              .trim_start_matches("~/")
              .split("/")
              .map(|s| s.to_string())
              .collect();
          } else {
            absolute_path = Some(value.value())
          }
        }
        _ => {
          dbg!(&meta);
          todo!();
        }
      },
    }
  }

  PageAttributes {
    absolute_path,
    path_segments,
    render_strategy: vec![],
    server_props,
  }
}

pub(crate) fn parse(item: ItemFn, attr: AttributeArgs) {
  create_dir_all(TARGET_PATH).unwrap();
  let attr = parse_attributes(attr);
  let ident = item.sig.ident;
  let props = item.sig.inputs.iter().collect::<Vec<_>>()[1..]
    .to_vec()
    .iter()
    .filter_map(|arg| match arg {
      syn::FnArg::Typed(arg) => {
        let ident = match arg.pat.as_ref() {
          syn::Pat::Ident(value) => value.ident.to_string(),
          _ => {
            dbg!(&arg.pat);
            todo!()
          }
        };
        let arg_ty = &arg.ty;
        let ty = quote!(#arg_ty).to_string();
        Some(FnArg { ident, ty })
      }
      syn::FnArg::Receiver(_) => None,
    })
    .collect::<Vec<_>>();
  let file_path = file_path_from_call_site();
  let parts = file_path.split("src/").collect::<Vec<_>>();
  let file_path = parts.get(1).unwrap().trim_end_matches(".rs");
  let path = {
    let path = match attr.absolute_path {
      Some(path) => path,
      None => file_path
        .trim_start_matches("pages/")
        .trim_end_matches("index")
        .to_owned(),
    };

    if attr.path_segments.is_empty() {
      path
    } else {
      let mut path = path.trim_end_matches("/").split("/").collect::<Vec<_>>();
      let _ = path.pop();
      format!(
        "{}/{}",
        path.join("/").replace("//", "/"),
        attr.path_segments.join("/")
      )
    }
  };
  let module_path = module_path_from_call_site();

  let page = if props.is_empty() && attr.server_props.is_none() {
    Route::StaticPage(StaticPageRoute {
      ident: ident.to_string(),
      module_path: module_path.clone(),
      path: format!("/{}", path.trim_start_matches("/").trim_end_matches("/")),
      render_strategy: attr.render_strategy,
    })
  } else {
    if props.is_empty() {
      abort_call_site!("For dynamic pages component `props` are required. Only #[inline_props] are supported at the moment.");
    }

    Route::DynamicPage(DynamicPageRoute {
      ident: ident.to_string(),
      module_path: module_path.clone(),
      props,
      path: format!("/{}", path.trim_start_matches("/").trim_end_matches("/")),
      render_strategy: attr.render_strategy,
      server_props: match attr.server_props {
        Some(value) => value,
        None => abort_call_site!("For dynamic pages `server_props` is required to be set."),
      },
    })
  };

  write_to_target(
    "route",
    &format!("{}-{}", &module_path, &ident.to_string()),
    page,
  );
}
