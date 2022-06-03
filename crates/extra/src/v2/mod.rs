use ahecha::html::{Attributes, Children, Component, Element, Node, NodeId};
use axum::{
  headers::{Error, Header, HeaderName, HeaderValue},
  http::StatusCode,
  response::{IntoResponse, Response},
  Json,
};
use axum_extra::routing::TypedPath;
use serde::Serialize;

pub enum Format {
  Html,
  Json,
  Partial,
}

impl Header for Format {
  fn name() -> &'static HeaderName {
    todo!()
  }

  fn decode<'i, I: Iterator<Item = &'i HeaderValue>>(_values: &mut I) -> Result<Self, Error> {
    todo!()
  }

  fn encode<E: Extend<HeaderValue>>(&self, _values: &mut E) {
    todo!()
  }
}

pub struct Router {
  routes: Vec<Route>,
}

impl Router {
  pub fn new() -> Self {
    Self { routes: vec![] }
  }

  pub fn add(mut self, route: Route) -> Self {
    self.routes.push(route);
    self
  }

  pub fn build_backend(self) -> axum::Router {
    let mut router = axum::Router::new();
    for r in self.routes.iter() {
      router = r.backend_route(router);
    }
    router
  }
}

pub struct Route {
  build_backend_handler: fn(router: axum::Router) -> axum::Router,
  routes: Vec<Route>,
}

impl Route {
  pub fn new() -> Self {
    Self {
      build_backend_handler: todo!(),
      routes: todo!(),
    }
  }

  pub fn backend_route(&self, router: axum::Router) -> axum::Router {
    let mut router = (self.build_backend_handler)(router);
    for r in self.routes.iter() {
      router = r.backend_route(router);
    }
    router
  }
}

pub trait Layout {
  type Slots;
  fn view<V>(self, slots: Self::Slots, view: V) -> Node
  where
    V: Into<Node>;
}

#[async_trait::async_trait]
pub trait Page<L>: Partial
where
  L: Layout,
{
  fn html_response(self, slots: <L as Layout>::Slots, layout: L) -> Response {
    layout.view(slots, self.view()).into_response()
  }
}

#[async_trait::async_trait]
pub trait Partial: Component + ComponentData + Sized {
  type Path: TypedPath;

  fn json_response(self) -> Response {
    (StatusCode::NOT_FOUND, self.data()).into_response()
  }

  fn partial_response(self) -> Response {
    self.view().into_response()
  }
}

pub trait ComponentData: Sized + Serialize {
  fn data(&self) -> Json<Self>;
}

pub struct PartialView<T, V>
where
  T: TypedPath,
  V: Component,
{
  path: T,
  view: V,
}

impl<T, V> Component for PartialView<T, V>
where
  T: TypedPath,
  V: Component,
{
  fn view(&self) -> Node {
    Node::Element(
      Element {
        attributes: Attributes::default().set(Some(("data-path", self.path.to_string()))),
        children: Children::default().set(self.view),
        name: "partial-view",
      },
      NodeId::new(),
    )
  }
}

pub trait PageError: Component + ComponentData {
  // fn status_code(&self) -> StatusCode;

  fn json_response(self) -> Response {
    (self.status_code(), self.data()).into_response()
  }

  fn partial_response(self) -> Response {
    self.view().into_response()
  }
}

pub trait ResponseExt<L = ()>
where
  L: Layout,
{
  fn into_formatted_response(
    self,
    format: Format,
    layout: Option<L>,
    slots: Option<L::Slots>,
  ) -> Response;
}

// impl<V, E> ResponseExt<()> for Result<V, E>
// where
//   V: Partial,
//   E: PageError,
// {
//   fn into_formatted_response(self, format: Format, _: (), _: ()) -> Response {
//     match format {
//       Format::Html => (StatusCode::NOT_FOUND, "not_found").into_response(),
//       Format::Json => match self {
//         Ok(s) => s.json_response(),
//         Err(e) => e.json_response(),
//       },
//       Format::Partial => match self {
//         Ok(s) => s.partial_response(),
//         Err(e) => e.partial_response(),
//       },
//     }
//   }
// }

impl<V, E, L> ResponseExt<L> for Result<V, E>
where
  V: Page<L>,
  E: PageError,
  L: Layout,
{
  fn into_formatted_response(
    self,
    format: Format,
    layout: Option<L>,
    slots: Option<L::Slots>,
  ) -> Response {
    match format {
      Format::Html => {
        if let (Some(layout), Some(slots)) = (layout, slots) {
          match self {
            Ok(s) => layout.view(slots, s).into_response(),
            Err(e) => (e.status_code(), layout.view(slots, e)).into_response(),
          }
        } else {
          (StatusCode::NOT_FOUND, "not found").into_response()
        }
      }
      Format::Json => match self {
        Ok(s) => s.json_response(),
        Err(e) => e.json_response(),
      },
      Format::Partial => match self {
        Ok(s) => s.partial_response(),
        Err(e) => e.partial_response(),
      },
    }
  }
}

impl Layout for () {
  type Slots = ();

  fn view<V>(self, _: Self::Slots, _: V) -> Node
  where
    V: Into<Node>,
  {
    unimplemented!()
  }
}

// pub trait Component {
// fn view<E>(self) -> Result<Node, E>
// where
// E: IntoNode;
// }

/**
#[derive(Page, serde::Deserialize)]
#[page(path = "/")] // nightly, if no path is provided uses the file path to determine the url path
struct HomePage;

#[async_trait::async_trait]
impl<B> Page<SomeLayout, B> for HomePage
where
  B: Send,
{
  type Path = HomePagePath;
  type Params = ();
  type Layout = ();
  async fn from_params<E>(path: <Self as Page<B>>::Path, params: <Self as Page<B>::Params) -> Result<(<Self as Page<B>>::Layout::Slots, Self), E> where E: IntoNode + IntoResponse {
    Ok(((), HomePage))
  }
}

impl Component for HomePage {
  fn view<E>(self) -> Result<Node, E>
  where
    E: IntoNode {
    Node::None
  }
}

// ------ autogenerated

#[derive(TypedPath, serde::Deserialize)]
pub struct HomePagePath {}

impl HomePage {
  fn into_json(self) -> axum::Response {
    Json(self).into_response()
  }
}

fn routes(router: Router) -> Router {
  router.add(|axum::Router| {
    router.typed_get(async |
      path: HomePage::Path,
      params: HomePage::Params,
      layout: HomePageLayout,
      TypedHeader(content_type): TypedHeader(headers::ContentType)
    |{
      let (slots, page) = HomePagePath::from_params(path, params).await;
      match content_type.as_str() {
        "application/json" => page.into_json(),
        "text/html" => layout.view(slots, page).into_response(),
        "text/html+partial" => page.into_response()
      }
    })
  })
}

// ----- macro ideas

// app/mod.rs
app!();

// ---- will generate a mod struct for all files under the `src/app` folder

mod page;
mod dashboard {
    mod page;
}

#[cfg(feature = "backend", not(feature = "frontend"))]
fn build() -> axum::Router {
  let router = Router::new();
  let router = mod::page::routes(router);
  let router = mod::dashboard::page::routes(router);
  router.build_backend()
}

#[cfg(feature = "frontend", not(feature = "backend"))]
fn build() -> Node {
  let router = Router::new();
  let router = mod::page::routes(router);
  let router = mod::dashboard::page::routes(router);
  router.build_frontend()
}
*/

#[test]
fn test() {
  let router = Router::new();
  let route = Route::new();
  let app = router.add(route).build_backend();
}
