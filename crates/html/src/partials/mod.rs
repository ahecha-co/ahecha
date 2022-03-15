use std::collections::HashMap;
use std::future::Future;

use async_trait::async_trait;
use axum_core::{
  extract::{rejection::HeadersAlreadyExtracted, FromRequest, RequestParts},
  response::{IntoResponse, Response},
};
use serde::Deserialize;

use crate::{Component, Node};

#[derive(Deserialize)]
struct PartialQuery {
  pub partial: Option<String>,
}

pub trait PartialView: Component {
  fn id(&self) -> String;
}

pub struct PartialBuilder {
  path: String,
  partials: HashMap<String, Node>,
}

impl PartialBuilder {
  pub fn new<T>(path: T) -> Self
  where
    T: ToString,
  {
    Self {
      path: path.to_string(),
      partials: HashMap::new(),
    }
  }

  pub fn render<P>(&mut self, partial: P) -> Node
  where
    P: PartialView,
  {
    let view = partial.view();
    self.partials.insert(partial.id(), view.clone());
    view
  }

  pub fn url_for<P>(&self, partial: &P) -> String
  where
    P: PartialView,
  {
    format!("{}?partial={}", &self.path, partial.id())
  }
}

pub struct PartialLayout {
  path: String,
  partial: Option<String>,
}

impl PartialLayout {
  pub fn url_for<P>(&self, partial: &P) -> String
  where
    P: PartialView,
  {
    format!("{}?partial={}", &self.path, partial.id())
  }

  pub fn render<F>(self, render: F) -> Node
  where
    F: FnOnce(&mut PartialBuilder) -> Node,
  {
    // TODO: find a way to register partials in the layout to avoid rendering them twice, this also will help in the future if we move the logic inside each component
    let mut builder = PartialBuilder {
      path: self.path.clone(),
      partials: Default::default(),
    };

    let view = render(&mut builder);

    if let Some(partial) = self.partial.as_ref() {
      if let Some(partial) = builder.partials.get(partial) {
        return partial.clone();
      }
    }

    view
  }

  pub async fn render_async<F, Fut>(self, render: F) -> Node
  where
    F: FnOnce(PartialBuilder) -> Fut,
    Fut: Future<Output = Node>,
  {
    let builder = PartialBuilder {
      path: self.path.clone(),
      partials: Default::default(),
    };

    let view = render(builder).await;

    /*
    if let Some(partial) = self.partial.as_ref() {
      if let Some(partial) = builder.partials.get(partial) {
        return partial.clone();
      }
    }
    */

    view
  }
}

#[async_trait]
impl<B> FromRequest<B> for PartialLayout
where
  B: Send,
{
  type Rejection = PartialLayoutRejection;

  /// Perform the extraction.
  async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
    let path = req.uri().path().to_owned();
    let query = req.uri().query().unwrap_or_default();
    let partial = match serde_urlencoded::from_str::<'_, PartialQuery>(query) {
      Ok(value) => value.partial,
      Err(_) => None,
    };

    Ok(Self { path, partial })
  }
}

pub enum PartialLayoutRejection {
  HeadersAlreadyExtracted(HeadersAlreadyExtracted),
}

impl IntoResponse for PartialLayoutRejection {
  fn into_response(self) -> Response {
    match self {
      Self::HeadersAlreadyExtracted(inner) => inner.into_response(),
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::{Children, Element, RenderString};

  #[test]
  fn test_partial_layout() {
    fn main_layout(_: &mut PartialBuilder) -> Node {
      Node::Element(Element {
        name: "div",
        attributes: Default::default(),
        children: Children::default().set(Node::Text("Hello world".to_owned())),
      })
    }

    let layout = PartialLayout {
      path: "/".to_owned(),
      partial: None,
    };

    let res = layout.render(main_layout);

    assert_eq!("<div>Hello world</div>", res.render());
  }

  #[test]
  fn test_render_partial() {
    struct PartialTest;

    impl PartialView for PartialTest {
      fn id(&self) -> String {
        "test".to_owned()
      }
    }

    impl Component for PartialTest {
      fn view(&self) -> Node {
        Node::Text(" I am a partial".to_owned())
      }
    }

    fn main_layout(inner: &mut PartialBuilder) -> Node {
      Node::Element(Element {
        name: "div",
        attributes: Default::default(),
        children: Children::default()
          .set(Node::Text("Hello world".to_owned()))
          .set(inner.render(PartialTest)),
      })
    }

    let layout = PartialLayout {
      path: "/".to_owned(),
      partial: None,
    };

    let res = layout.render(main_layout);

    assert_eq!("<div>Hello world I am a partial</div>", res.render());

    let layout_partial = PartialLayout {
      path: "/".to_owned(),
      partial: Some("test".to_owned()),
    };

    assert_eq!("/?partial=test", layout_partial.url_for(&PartialTest));

    let res = layout_partial.render(main_layout);

    assert_eq!(" I am a partial", res.render());
  }
}
