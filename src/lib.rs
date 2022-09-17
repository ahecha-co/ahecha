use std::collections::HashMap;

pub use ahecha_macros::*;
use dioxus::prelude::*;

// TODO: Implement link
// TODO: Implement CSR router
// TODO: Implement CSR route

#[allow(dead_code)]
pub struct RouterService {
  initial_url: Option<String>,
  routes: HashMap<String, ()>,
}

impl RouterService {
  fn new(routes: HashMap<String, ()>) -> Self {
    #[cfg(not(target_arch = "wasm32"))]
    let initial_url = None;

    #[cfg(target_arch = "wasm32")]
    let initial_url = Some(
      web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .location()
        .unwrap()
        .href()
        .unwrap(),
    );

    Self {
      initial_url,
      routes,
    }
  }
}

pub struct Router {
  routes: HashMap<String, ()>,
}

impl Router {
  pub fn get(mut self, route: &str, page: ()) -> Self {
    // TODO: panic if route is already set
    self.routes.insert(route.to_owned(), page);
    self
  }

  pub fn into_make_service(self) -> RouterService {
    RouterService::new(self.routes)
  }
}

#[allow(non_snake_case)]
pub fn RouterComponent(cx: Scope) -> Element {
  let _service = use_context::<RouterService>(&cx)?;
  cx.render(rsx!("Router"))
}
