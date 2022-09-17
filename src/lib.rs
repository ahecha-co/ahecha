use std::{cell::RefCell, rc::Rc, sync::Arc};

pub use ahecha_macros::*;
use dioxus::prelude::*;

#[derive(Clone)]
pub struct RouterContext {
  pub location: Option<String>,
}

impl RouterContext {
  pub fn new(location: &Option<&str>) -> Self {
    let location = match location {
      Some(location) => Some(location.to_string()),
      None => {
        #[cfg(target_arch = "wasm32")]
        match web_sys::window() {
          Some(window) => match window.document() {
            Some(document) => match document.location() {
              Some(location) => match location.pathname() {
                Ok(pathname) => Some(pathname),
                Err(_) => None,
              },
              None => None,
            },
            None => None,
          },
          None => None,
        }

        #[cfg(not(target_arch = "wasm32"))]
        None
      }
    };

    Self { location }
  }
}

#[derive(Clone)]
pub struct RoutesContext {
  base_path: Option<String>,
  router: matchit::Router<Component>,
}

impl RoutesContext {
  pub fn new(base_path: Option<String>) -> Self {
    Self {
      base_path,
      router: matchit::Router::new(),
    }
  }
}

#[allow(non_snake_case)]
#[inline_props]
pub fn BrowserRouter<'a>(
  cx: Scope<'a>,
  location: Option<&'a str>,
  children: Element<'a>,
) -> Element<'a> {
  use_context_provider(&cx, || RouterContext::new(location));
  cx.render(rsx!(children))
}

#[allow(non_snake_case)]
pub fn Empty(cx: Scope) -> Element {
  tracing::trace!("Rendering empty component");
  None
}

#[derive(Props)]
pub struct RoutesProps<'a> {
  base_path: Option<&'a str>,
  children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn Routes<'a>(cx: Scope<'a, RoutesProps<'a>>) -> Element<'a> {
  use_context_provider(&cx, || {
    RoutesContext::new(cx.props.base_path.map(|s| s.to_string()))
  });
  let context = use_context::<RoutesContext>(&cx)?;
  let router_context = use_context::<RouterContext>(&cx)?;

  tracing::trace!("Searching for a components to show");
  let MatchedComponent = match router_context.read().location.as_ref() {
    Some(location) => match context.read().router.at(location.as_str()) {
      Ok(res) => {
        tracing::trace!("A route matched");
        res.value.clone()
      }
      Err(err) => {
        tracing::error!("{}", err);
        Empty
      }
    },
    None => {
      tracing::trace!("No location found, to show a component a location must be provided");
      Empty
    }
  };

  cx.render(rsx!(
    &cx.props.children
    MatchedComponent {}
  ))
}

#[derive(Clone)]
pub struct RouteContext {
  absolute_path: String,
  relative_path: String,
}

#[allow(non_snake_case)]
#[inline_props]
pub fn Route<'a>(
  cx: Scope<'a>,
  path: &'a str,
  children: Element<'a>,
  element: Component,
) -> Element<'a> {
  let context = use_context::<RoutesContext>(&cx)?;

  cx.use_hook(|| {
    tracing::trace!("Registering route: {}", path);
    let absolute_path = match cx.consume_context::<RouteContext>() {
      Some(parent_context) => format!(
        "{}/{}",
        parent_context.absolute_path.trim_end_matches("/"),
        path
      ),
      None => path.to_string(),
    };

    let route_context = cx.provide_context(RouteContext {
      absolute_path,
      relative_path: path.to_string(),
    });

    context
      .write()
      .router
      .insert(route_context.absolute_path, element.clone());
  });

  cx.render(rsx!(children))
}
