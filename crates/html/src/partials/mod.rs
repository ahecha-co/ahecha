use std::collections::HashMap;

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
  fn id() -> &'static str;
}

pub struct PartialInner {
  partials: HashMap<String, Node>,
}

impl PartialInner {
  pub fn render<P>(&mut self, partial: P) -> Node
  where
    P: PartialView,
  {
    let view = partial.view();
    self.partials.insert(P::id().to_owned(), view.clone());
    view
  }
}

pub struct PartialLayout {
  inner: PartialInner,
  path: String,
  partial: Option<String>,
}

impl PartialLayout {
  pub fn render<F>(self, render: F) -> Node
  where
    F: FnOnce(&PartialInner) -> Node,
  {
    if let Some(partial) = self.partial.as_ref() {
      if let Some(partial) = self.inner.partials.get(partial) {
        return partial.clone();
      }
    }

    render(&self.inner)
  }

  pub fn url_for<P>(&self) -> String
  where
    P: PartialView,
  {
    format!("{}?partial={}", &self.path, P::id())
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

    Ok(Self {
      inner: PartialInner {
        partials: HashMap::new(),
      },
      path,
      partial,
    })
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
