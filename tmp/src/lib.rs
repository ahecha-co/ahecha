pub use ahecha_macros::*;
use dioxus::prelude::*;

pub trait RouterHistory {
  fn back(&mut self);
  fn forward(&mut self);
  fn push(&mut self, url: impl AsRef<str>);
  fn replace(&mut self, url: impl AsRef<str>);
}

#[derive(Clone)]
pub struct RouterCore {
  pub location: Option<String>,
  #[cfg(target_arch = "wasm32")]
  history: web_sys::History,
}

impl RouterCore {
  pub fn new(location: &Option<&str>) -> Self {
    let location = match location {
      Some(location) => Some(location.to_string()),
      None => {
        #[cfg(target_arch = "wasm32")]
        match web_sys::window() {
          Some(window) => match window.location().pathname() {
            Ok(pathname) => Some(pathname),
            Err(_) => None,
          },
          None => None,
        }

        #[cfg(not(target_arch = "wasm32"))]
        None
      }
    };

    Self {
      location,
      #[cfg(target_arch = "wasm32")]
      history: web_sys::window().unwrap().history().unwrap(),
    }
  }
}

impl RouterHistory for RouterCore {
  fn push(&mut self, url: impl AsRef<str>) {
    // #[cfg(not(target_arch = "wasm32"))]
    {
      self.location = Some(url.as_ref().to_owned());
    }

    #[cfg(target_arch = "wasm32")]
    if let Err(err) =
      self
        .history
        .push_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(url.as_ref()))
    {
      tracing::error!("{:?}", &err);
    }
  }

  fn replace(&mut self, url: impl AsRef<str>) {
    // #[cfg(not(target_arch = "wasm32"))]
    {
      self.location = Some(url.as_ref().to_owned());
    }

    #[cfg(target_arch = "wasm32")]
    if let Err(err) =
      self
        .history
        .replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(url.as_ref()))
    {
      tracing::error!("{:?}", &err);
    }
  }

  fn back(&mut self) {
    todo!()
  }

  fn forward(&mut self) {
    todo!()
  }
}

#[derive(Clone)]
pub struct RoutesContext {
  active_class: String,
  base_path: String,
  fallback: Option<Component>,
  router: matchit::Router<Component>,
}

impl RoutesContext {
  pub fn new(base_path: &str, active_class: &Option<&str>) -> Self {
    Self {
      active_class: active_class.map_or_else(|| "active".to_owned(), |s| s.to_owned()),
      base_path: base_path.to_owned(),
      fallback: None,
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
  use_context_provider(&cx, || RouterCore::new(&location));
  // let context = use_context::<RouterCore>(&cx)?;
  // cx.use_hook(|| {
  //   // TODO: figure out how to update the location
  //   #[cfg(target_arch = "wasm32")]
  //   {
  //     let context = std::sync::Arc::new(context);
  //     gloo::events::EventListener::new(&web_sys::window().unwrap(), "popstate", move |_| {
  //       let location = web_sys::window().unwrap().location().pathname().unwrap();
  //       tracing::trace!("window.popstate. Location {}", location);
  //       context.read().replace(location);
  //     });
  //   }
  // });
  cx.render(rsx!(children))
}

#[allow(non_snake_case)]
#[inline_props]
pub fn Fallback<'a>(cx: Scope<'a>, element: Component, children: Element<'a>) -> Element {
  let context = use_context::<RoutesContext>(&cx)?;
  let _ = children;

  cx.use_hook(|| {
    tracing::trace!("Registering fallback component");
    context.write().fallback = Some(element.clone());
  });

  None
}

#[allow(non_snake_case)]
#[inline_props]
fn InternalError(cx: Scope<'a>, error: String) -> Element {
  cx.render(rsx!(
    div {
      style: r#"
        background-color: rgb(254 242 242);
        border-radius: .375rem;
        padding: 1rem;
      "#,
      div {
        style: "display: flex",
        div {
          style: "flex-shrink: 0",
          svg {
            style: r#"
              color: rgb(248 113 113);
              height: 1.25rem;
              width: 1.25rem;
            "#,
            xmlns:"http://www.w3.org/2000/svg",
            view_box: "0 0 20 20",
            fill:"currentColor",
            path {
              fill_rule:"evenodd",
              d:"M10 18a8 8 0 100-16 8 8 0 000 16zM8.28 7.22a.75.75 0 00-1.06 1.06L8.94 10l-1.72 1.72a.75.75 0 101.06 1.06L10 11.06l1.72 1.72a.75.75 0 101.06-1.06L11.06 10l1.72-1.72a.75.75 0 00-1.06-1.06L10 8.94 8.28 7.22z",
              clip_rule: "evenodd",
            }
          }
        }
        div {
          style: "margin-left: 0.75rem;",
          p {
            style: r#"
              color: rgb(153 27 27);
              font-weight: 500;
              font-size: 0.875rem;
              line-height: 1.25rem;
              margin: 0;
            "#,
            "{error}"
          }
        }
      }
    }
  ))
}

#[allow(non_snake_case)]
#[inline_props]
pub fn Link<'a>(cx: Scope<'a>, to: &'a str, children: Element<'a>) -> Element {
  let navigate = use_navigate(&cx);
  cx.render(rsx!(a {
    href: "{to}",
    prevent_default: "onclick",
    onclick: move |_| navigate(to),
    children
  }))
}

#[allow(non_snake_case)]
#[inline_props]
pub fn NavLink<'a>(
  cx: Scope<'a>,
  to: &'a str,
  active_class: Option<&'a str>,
  children: Element<'a>,
) -> Element {
  let router_core = use_context::<RouterCore>(&cx)?;
  let context = use_context::<RoutesContext>(&cx);

  if let Some(context) = context {
    let active_router = use_state(&cx, || {
      let mut active_router = matchit::Router::new();
      active_router.insert(to.to_string(), true).unwrap();
      active_router
    });
    let class = if active_router
      .get()
      .at(
        &router_core
          .read()
          .location
          .clone()
          .unwrap_or_else(|| "".to_owned()),
      )
      .is_ok()
    {
      active_class.map_or_else(|| context.read().active_class.clone(), |s| s.to_owned())
    } else {
      "".to_owned()
    };
    let navigate = use_navigate(&cx);
    cx.render(rsx!(a {
      href: "{to}",
      class: "{class}",
      prevent_default: "onclick",
      onclick: move |_| navigate(to),
      children
    }))
  } else {
    cx.render(rsx!(InternalError {
      error: "`NavLink` can be used only as a child of `Routes`".to_owned()
    }))
  }
}

#[derive(Props)]
pub struct RoutesProps<'a> {
  active_class: Option<&'a str>,
  #[props(default)]
  base_path: &'a str,
  children: Element<'a>,
}

#[allow(non_snake_case)]
pub fn Routes<'a>(cx: Scope<'a, RoutesProps<'a>>) -> Element<'a> {
  let RoutesProps {
    active_class,
    base_path,
    children,
  } = &cx.props;
  use_context_provider(&cx, || RoutesContext::new(base_path, active_class));
  let context = use_context::<RoutesContext>(&cx)?;
  let router_core = use_context::<RouterCore>(&cx)?;
  let base_path = &context.read().base_path;

  cx.render(rsx!(
    children

    match router_core.read().location.as_ref() {
      Some(location) => match context
        .read()
        .router
        .at(location.as_str().trim_start_matches(base_path))
      {
        Ok(res) => {
          tracing::trace!("A route matched");
          let C = res.value.clone();
          rsx!( C {} )
        }
        Err(err) => {
          tracing::error!("{:?}", &err);
          match context.read().fallback {
            Some(Fallback) => rsx!(Fallback {}),
            None => rsx!(Fragment {}),
          }
        }
      },
      None => {
        rsx!(InternalError { error: "`RouterCore.location` is not set".to_owned() })
      }
    }
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
  let error = use_state(&cx, || None);

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

    if let Err(err) = context
      .write()
      .router
      .insert(route_context.absolute_path, element.clone())
    {
      tracing::error!("{:?}", err);
      error.set(Some(err.to_string()));
    }
  });

  cx.render(rsx!(
    children
    error.get().as_ref().map(|e| rsx!(InternalError { error: e.clone() }))
  ))
}

pub fn use_navigate(cx: &ScopeState) -> impl FnOnce(&str) + '_ + Copy {
  let context = use_context::<RouterCore>(&cx)
    .expect("`use_navigate` can be used in components wraped by `BrowserRouter`");
  move |path| context.write().push(path)
}
