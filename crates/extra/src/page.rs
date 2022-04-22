// TODO: Experiment again with pages, but this time also with getServerSideProps and getStaticProps, and automatically generate the page route.
// Maybe infer the route if the route attribute is missing?
// Open question, how to register the route? Make it a macro? Auto generate code with [tests](https://matklad.github.io/2022/03/26/self-modifying-code.html)?

use crate::view::Scope;
use ahecha_html::Node;
use axum_extra::routing::TypedPath;
use http::StatusCode;

use crate::{HttpMethod, Layout, PageScope};

pub type ViewError = (StatusCode, Node, Node);

#[axum::async_trait]
pub trait Page: NestedPage + Sync + Send {
  type Layout: Layout;

  async fn slots(&self) -> <Self::Layout as Layout>::Slots;
}

#[axum::async_trait]
pub trait NestedPage {
  type Path: TypedPath;

  fn methods() -> &'static [&'static HttpMethod] {
    &[&HttpMethod::GET]
  }

  async fn render(&self, scope: PageScope) -> Result<(PageScope, Node), ViewError>;
}

pub trait Component {
  fn render(&self, scope: Scope) -> Node;
}

#[cfg(test)]
mod test {
  use super::*;
  use ahecha_macro::html;
  use axum::{
    response::{IntoResponse, Response},
    Router,
  };
  use axum_extra::routing::RouterExt;
  use axum_macros::FromRequest;

  #[derive(FromRequest)]
  struct AppLayout;

  #[axum::async_trait]
  impl Layout for AppLayout {
    type Error = ();
    type Slots = ();

    fn render(&self, _: Self::Slots, body: Node) -> Node {
      html!(<div>{ body }</div>)
    }
  }

  #[derive(TypedPath)]
  #[typed_path("/")]
  struct HomePagePath;

  struct HomePage;

  #[axum::async_trait]
  impl Page for HomePage {
    type Layout = AppLayout;
    async fn slots(&self) -> <Self::Layout as Layout>::Slots {
      ()
    }
  }

  #[axum::async_trait]
  impl NestedPage for HomePage {
    type Path = HomePagePath;

    async fn render(&self, scope: PageScope) -> Result<(PageScope, Node), ViewError> {
      Ok((scope, html!(<div> Home page content </div>)))
    }
  }

  impl From<HomePage> for Router {
    fn from(_: HomePage) -> Router {
      async fn handler(
        _: <HomePage as NestedPage>::Path,
        layout: <HomePage as Page>::Layout,
      ) -> Response {
        let scope = PageScope::default();
        let page = HomePage;
        let (status, body) = match page.render(scope).await {
          Ok((scope, body)) => (scope.status(), body),
          Err((status, body, _)) => (status, body),
        };

        (status, layout.render(page.slots().await, body)).into_response()
      }

      let mut router = Router::new();

      for method in <HomePage as NestedPage>::methods().iter() {
        router = match method {
          HttpMethod::DELETE => router.typed_delete(handler),
          HttpMethod::GET => router.typed_get(handler),
          HttpMethod::PATCH => router.typed_patch(handler),
          HttpMethod::POST => router.typed_post(handler),
          HttpMethod::PUT => router.typed_put(handler),
        }
      }

      router
    }
  }

  #[derive(TypedPath)]
  #[typed_path("/partial/test")]
  struct TestPartialPath;

  struct TestPartial;

  #[axum::async_trait]
  impl NestedPage for TestPartial {
    type Path = TestPartialPath;

    async fn render(&self, scope: PageScope) -> Result<(PageScope, Node), ViewError> {
      Ok((scope, html!(<div> I am a partial</div>)))
    }
  }

  impl From<TestPartial> for Router {
    fn from(_: TestPartial) -> Self {
      async fn handler(
        _: <TestPartial as NestedPage>::Path,
        scope: PageScope,
      ) -> impl IntoResponse {
        let page = TestPartial {};
        match page.render(scope).await {
          Ok((scope, body)) => (scope.status(), body),
          Err((status, body, _)) => (status, body),
        }
      }

      let mut router = Router::new();

      for method in <TestPartial as NestedPage>::methods().iter() {
        router = match method {
          HttpMethod::DELETE => router.typed_delete(handler),
          HttpMethod::GET => router.typed_get(handler),
          HttpMethod::PATCH => router.typed_patch(handler),
          HttpMethod::POST => router.typed_post(handler),
          HttpMethod::PUT => router.typed_put(handler),
        }
      }

      router
    }
  }

  fn routes() {
    Router::new().merge(HomePage).merge(TestPartial);
  }

  #[test]
  fn test_page() {
    assert!(false);
  }
}
