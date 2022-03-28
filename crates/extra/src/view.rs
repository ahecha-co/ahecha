use std::collections::HashMap;

use ahecha_html::{Children, Node};
use axum::extract::{FromRequest, RequestParts};
use dyn_clone::DynClone;
use serde::Deserialize;

use crate::PageComponent;

#[derive(Default)]
pub struct PartialManager {
  path: String,
  partials: HashMap<String, Box<dyn PartialView>>,
}

impl PartialManager {
  pub fn new<T>(path: T) -> Self
  where
    T: ToString,
  {
    Self {
      path: path.to_string(),
      partials: HashMap::new(),
    }
  }

  pub fn get(&self, id: &str) -> Option<&Box<dyn PartialView>> {
    self.partials.get(id)
  }

  pub async fn render<P>(&mut self, partial: P, scope: &mut Scope) -> Node
  where
    P: PartialView + 'static,
  {
    let boxed = Box::new(partial);
    self
      .partials
      .insert(boxed.id(), dyn_clone::clone_box(&*boxed));
    boxed.view(scope).await
  }

  pub fn url_for<P>(&self, partial: &P) -> String
  where
    P: PartialView,
  {
    format!("{}?partial={}", &self.path, partial.id())
  }
}

pub trait PartialView: Component + DynClone + Send + Sync {
  fn id(&self) -> String;
}

#[axum::async_trait]
pub trait Component {
  async fn view(&self, scope: &mut Scope) -> Node;
}

pub trait ErrorComponent {
  fn view(&self) -> Node;
  fn error(&self) -> Node;
}

impl ErrorComponent for () {
  fn view(&self) -> Node {
    Node::None
  }

  fn error(&self) -> Node {
    Node::None
  }
}

// TODO: Find an elegant way to require FromRequest<T> trait to be implemented for Layout
pub trait Layout {
  type Error: ErrorComponent;
  type Slots: Default;
  fn can_render_errors(&self) -> bool;
  fn render(&self, slots: Self::Slots, body: Node) -> Node;
}

#[derive(Deserialize)]
struct PartialQuery {
  pub partial: Option<String>,
}

pub struct View<L>
where
  L: Layout,
{
  layout: L,
  partial_id: Option<String>,
  path: String,
}

impl<L> View<L>
where
  L: Layout,
{
  pub async fn render<P>(&mut self, page: P) -> Node
  where
    P: PageComponent<L>,
  {
    let mut scope = Scope {
      partial_id: self.partial_id.clone(),
      partials: PartialManager::new(&self.path),
    };
    let body = match page.view(&mut scope).await {
      Ok(body) => body,
      Err(err) => {
        if self.layout.can_render_errors() {
          Node::Fragment(Children::default().set(err.view()).set(err.error()))
        } else {
          err.view()
        }
      }
    };

    if let Some(partial_id) = self.partial_id.as_ref() {
      if let Some(partial) = scope.partials.get(partial_id) {
        return partial.view(&mut scope.clone()).await;
      }
    }

    self.layout.render(page.slots().await, body)
  }
}

#[axum::async_trait]
impl<B, L> FromRequest<B> for View<L>
where
  B: Send, // required by `async_trait`
  L: Layout + FromRequest<B, Rejection = http::StatusCode>,
{
  type Rejection = http::StatusCode;

  async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
    let layout = L::from_request(req).await?;
    let path = req.uri().path().to_owned();
    let query = req.uri().query().unwrap_or_default();
    let partial_id = match serde_urlencoded::from_str::<'_, PartialQuery>(query) {
      Ok(value) => value.partial,
      Err(_) => None,
    };

    Ok(View {
      layout,
      partial_id,
      path,
    })
  }
}

#[derive(Default)]
pub struct Scope {
  partial_id: Option<String>,
  partials: PartialManager,
}

impl Scope {
  pub fn new() -> Self {
    Default::default()
  }

  pub async fn partial<P>(&mut self, partial: P) -> Node
  where
    P: PartialView + 'static,
  {
    // TODO: Prevent rendering other partials when the partial_id is known, we might need to check for nested partials
    if let Some(partial_id) = self.partial_id.as_ref() {
      if partial_id != &partial.id() {
        return Node::None;
      }
    }

    self.partials.render(partial, &mut self.clone()).await
  }
}

impl Clone for Scope {
  fn clone(&self) -> Self {
    Self {
      partial_id: self.partial_id.clone(),
      partials: Default::default(),
    }
  }
}

pub type Element = Node;

#[cfg(test)]
mod test {
  use ahecha_html::{Children, RenderString};
  use axum::{
    extract::FromRequest,
    response::IntoResponse,
    routing::{get, Router},
  };
  use http::StatusCode;

  use super::*;

  #[tokio::test]
  async fn test_app() {
    #[derive(axum_macros::FromRequest)]
    struct TestLayout;

    impl Layout for TestLayout {
      type Error = ();
      type Slots = ();

      fn can_render_errors(&self) -> bool {
        true
      }

      fn render(&self, _slots: Self::Slots, body: Node) -> Node {
        body
      }
    }

    let mut view = View {
      layout: TestLayout,
      partial_id: None,
      path: "/".to_owned(),
    };

    struct TestPage;

    #[axum::async_trait]
    impl PageComponent<TestLayout> for TestPage {
      async fn view(&self, _scope: &mut Scope) -> Result<Node, <TestLayout as Layout>::Error> {
        Ok(Node::Element(ahecha_html::Element {
          name: "div",
          attributes: Default::default(),
          children: Children::default().set(Node::Text("Hello".to_owned())),
        }))
      }
    }

    let el = view.render(TestPage).await;
    assert_eq!(el.render(), "<div>Hello</div>");
  }

  #[tokio::test]
  async fn test_partial() {
    #[derive(axum_macros::FromRequest)]
    struct TestLayout;

    impl Layout for TestLayout {
      type Error = ();
      type Slots = ();

      fn can_render_errors(&self) -> bool {
        true
      }

      fn render(&self, _slots: Self::Slots, body: Node) -> Node {
        body
      }
    }
    #[derive(Clone)]
    struct TestPartial;

    impl PartialView for TestPartial {
      fn id(&self) -> String {
        "test".to_owned()
      }
    }

    #[async_trait::async_trait]
    impl Component for TestPartial {
      async fn view(&self, _: &mut Scope) -> Node {
        Node::Text("I am a partial".to_owned())
      }
    }

    let mut view = View {
      layout: TestLayout,
      partial_id: Some("test".to_owned()),
      path: "/".to_owned(),
    };

    struct TestPage;

    #[axum::async_trait]
    impl PageComponent<TestLayout> for TestPage {
      async fn view(&self, scope: &mut Scope) -> Result<Node, <TestLayout as Layout>::Error> {
        Ok(Node::Element(ahecha_html::Element {
          name: "div",
          attributes: Default::default(),
          children: Children::default()
            .set(Node::Text("Hello".to_owned()))
            .set(scope.partial(TestPartial).await),
        }))
      }
    }

    let el = view.render(TestPage).await;
    assert_eq!(el.render(), "I am a partial");
  }

  #[tokio::test]
  async fn test_axum_route() {
    struct TestLayout;

    impl Layout for TestLayout {
      type Error = ();
      type Slots = ();

      fn can_render_errors(&self) -> bool {
        true
      }

      fn render(&self, _slots: Self::Slots, body: Node) -> Node {
        body
      }
    }

    #[axum::async_trait]
    impl<B> FromRequest<B> for TestLayout
    where
      B: Send, // required by `async_trait`
    {
      type Rejection = http::StatusCode;

      async fn from_request(_: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        Ok(TestLayout)
      }
    }

    async fn handler(_: View<TestLayout>) -> impl IntoResponse {
      StatusCode::OK
    }

    Router::<StatusCode>::new().route("/", get(handler));
  }
}
