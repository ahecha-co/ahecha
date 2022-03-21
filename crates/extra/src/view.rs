use std::{collections::HashMap, future::Future};

use ahecha_html::Node;
use axum::extract::{FromRequest, RequestParts};
use dyn_clone::DynClone;
use serde::Deserialize;

pub struct Partial {
  path: String,
  partials: HashMap<String, Box<dyn PartialView>>,
}

impl Partial {
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

  pub async fn render<P>(&mut self, partial: P) -> Node
  where
    P: PartialView + 'static,
  {
    let boxed: Box<dyn PartialView> = Box::new(partial);
    self
      .partials
      .insert(boxed.id(), dyn_clone::clone_box(&*boxed));
    boxed.view().await
  }

  pub fn url_for<P>(&self, partial: &P) -> String
  where
    P: PartialView,
  {
    format!("{}?partial={}", &self.path, partial.id())
  }
}

pub trait PartialView: Component + DynClone {
  fn id(&self) -> String;
}

#[axum::async_trait]
pub trait Component {
  async fn view(&self) -> Node;
}

// TODO: Find an elegant way to require FromRequest<T> trait to be implemented for Layout
pub trait Layout {
  type Props: Clone;
  type Slots: Default;
  fn render<'a, P>(&self, scope: Scope<P, Self::Slots>, body: Node) -> Node;
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
  pub async fn render<F, P, O>(&mut self, view: F, props: P) -> Node
  where
    F: Fn(Scope<P, L::Slots>) -> O,
    O: Future<Output = Element<P, L::Slots>>,
  {
    let scope = Scope {
      partial_id: self.partial_id.clone(),
      partials: Partial::new(&self.path),
      props,
      slots: Default::default(),
    };
    let (body, scope) = view(scope).await;

    if let Some(partial_id) = self.partial_id.as_ref() {
      if let Some(partial) = scope.partials.get(partial_id) {
        return partial.view().await;
      }
    }

    self.layout.render(scope, body)
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

pub struct Scope<Props = (), S = ()>
where
  S: Default,
{
  partial_id: Option<String>,
  partials: Partial,
  pub props: Props,
  pub slots: S,
}

impl<Props, S> Scope<Props, S>
where
  S: Default,
{
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

    self.partials.render(partial).await
  }
}

pub type Element<P = (), S = ()> = (Node, Scope<P, S>);

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
    struct TestLayout;

    impl Layout for TestLayout {
      type Props = ();
      type Slots = ();

      fn render<'a, P>(&self, _scope: Scope<P>, body: Node) -> Node {
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

    let mut view_engine = View {
      layout: TestLayout,
      partial_id: None,
      path: "/".to_owned(),
    };

    async fn view(scope: Scope) -> Element {
      let (_, scope) = view2(scope).await;
      let b = view2(scope).await;
      b
    }

    async fn view2(scope: Scope) -> Element {
      (
        Node::Element(ahecha_html::Element {
          name: "div",
          attributes: Default::default(),
          children: Children::default().set(Node::Text("Hello".to_owned())),
        }),
        scope,
      )
    }

    let el = view_engine.render(view, ()).await;
    assert_eq!(el.render(), "<div>Hello</div>");
  }

  #[tokio::test]
  async fn test_partial() {
    struct TestLayout;

    impl Layout for TestLayout {
      type Props = ();
      type Slots = ();

      fn render<'a, P>(&self, _scope: Scope<P>, body: Node) -> Node {
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

    #[derive(Clone)]
    struct TestPartial;

    impl PartialView for TestPartial {
      fn id(&self) -> String {
        "test".to_owned()
      }
    }

    #[async_trait::async_trait]
    impl Component for TestPartial {
      async fn view(&self) -> Node {
        Node::Text("I am a partial".to_owned())
      }
    }

    let mut view_engine = View {
      layout: TestLayout,
      partial_id: Some("test".to_owned()),
      path: "/".to_owned(),
    };

    async fn view(mut scope: Scope) -> Element {
      (
        Node::Element(ahecha_html::Element {
          name: "div",
          attributes: Default::default(),
          children: Children::default()
            .set(Node::Text("Hello".to_owned()))
            .set(scope.partial(TestPartial).await),
        }),
        scope,
      )
    }

    let el = view_engine.render(view, ()).await;
    assert_eq!(el.render(), "I am a partial");
  }

  #[tokio::test]
  async fn test_axum_route() {
    struct TestLayout;

    impl Layout for TestLayout {
      type Props = ();
      type Slots = ();

      fn render<'a, P>(&self, _scope: Scope<P>, body: Node) -> Node {
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
