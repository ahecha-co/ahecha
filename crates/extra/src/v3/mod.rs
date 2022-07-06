use ahecha::html::{IntoNode, Node};
use axum::{
  extract::FromRequest,
  http::{
    header::{self},
    StatusCode,
  },
  response::{IntoResponse, Response},
  Json,
};
use serde::Serialize;

pub trait Layout {
  fn view<B>(self, body: B) -> Node
  where
    B: IntoNode;
}

impl Layout for () {
  fn view<B>(self, body: B) -> Node
  where
    B: IntoNode,
  {
    body.into_node()
  }
}

fn get_format_from_request<B>(req: &mut axum::extract::RequestParts<B>) -> (&str, &str)
where
  B: Send,
{
  (
    if let Some(value) = req.headers().get(header::CONTENT_TYPE) {
      value.to_str().unwrap_or("")
    } else {
      ""
    },
    if let Some(value) = req.headers().get(header::ACCEPT) {
      value.to_str().unwrap_or("")
    } else {
      ""
    },
  )
}

fn supports_content_type<B>(
  req: &mut axum::extract::RequestParts<B>,
  supported_content_type: &str,
) -> bool
where
  B: Send,
{
  let (content_type, accepts) = get_format_from_request(req);

  content_type.contains(supported_content_type) || accepts.contains(supported_content_type)
}

#[derive(Debug)]
pub enum FormatResponse<L>
where
  L: Layout,
{
  Html(L),
  HtmlPartial,
  Json,
}

impl<L> FormatResponse<L>
where
  L: Layout,
{
  pub fn render<V>(self, view: V) -> Response
  where
    V: IntoNode + Serialize,
  {
    match self {
      FormatResponse::Html(layout) => layout.view(view).into_response(),
      FormatResponse::HtmlPartial => view.into_node().into_response(),
      FormatResponse::Json => Json(view).into_response(),
    }
  }
}

#[async_trait::async_trait]
impl<L, B> FromRequest<B> for FormatResponse<L>
where
  B: Send,
  L: Layout + FromRequest<B>,
{
  type Rejection = (StatusCode, String);

  async fn from_request(req: &mut axum::extract::RequestParts<B>) -> Result<Self, Self::Rejection> {
    if supports_content_type(req, "json") {
      Ok(FormatResponse::Json)
    } else if supports_content_type(req, "partial") {
      Ok(FormatResponse::HtmlPartial)
    } else if supports_content_type(req, "html") {
      let layout = req.extract::<L>().await.map_err(|_| {
        (
          StatusCode::INTERNAL_SERVER_ERROR,
          "Couldn't extract the layout from request".to_string(),
        )
      })?;
      Ok(FormatResponse::Html(layout))
    } else {
      Err((StatusCode::NOT_FOUND, "Not found".to_owned()))
    }
  }
}

pub enum HtmlResponse<L>
where
  L: Layout,
{
  Html(L),
  HtmlPartial,
}

impl<L> HtmlResponse<L>
where
  L: Layout,
{
  pub fn render<V>(self, view: V) -> Response
  where
    V: IntoNode,
  {
    match self {
      HtmlResponse::Html(layout) => layout.view(view).into_response(),
      HtmlResponse::HtmlPartial => view.into_node().into_response(),
    }
  }
}

#[async_trait::async_trait]
impl<L, B> FromRequest<B> for HtmlResponse<L>
where
  B: Send,
  L: Layout + FromRequest<B>,
{
  type Rejection = (StatusCode, String);

  async fn from_request(req: &mut axum::extract::RequestParts<B>) -> Result<Self, Self::Rejection> {
    if supports_content_type(req, "partial") {
      Ok(HtmlResponse::HtmlPartial)
    } else if supports_content_type(req, "html") {
      let layout = req.extract::<L>().await.map_err(|_| {
        (
          StatusCode::INTERNAL_SERVER_ERROR,
          "Couldn't extract the layout from request".to_string(),
        )
      })?;
      Ok(HtmlResponse::Html(layout))
    } else {
      Err((StatusCode::NOT_FOUND, "Not found".to_owned()))
    }
  }
}

pub struct HtmlPartialResponse;

impl HtmlPartialResponse {
  pub fn render<V>(self, view: V) -> Response
  where
    V: IntoNode,
  {
    view.into_node().into_response()
  }
}

pub enum PartialResponse {
  Html,
  Json,
}

impl PartialResponse {
  pub fn render<V>(self, view: V) -> Response
  where
    V: IntoNode + Serialize,
  {
    match self {
      PartialResponse::Html => view.into_node().into_response(),
      PartialResponse::Json => Json(view).into_response(),
    }
  }
}

#[async_trait::async_trait]
impl<B> FromRequest<B> for HtmlPartialResponse
where
  B: Send,
{
  type Rejection = (StatusCode, String);

  async fn from_request(req: &mut axum::extract::RequestParts<B>) -> Result<Self, Self::Rejection> {
    if supports_content_type(req, "partial") {
      Ok(HtmlPartialResponse)
    } else {
      Err((StatusCode::NOT_FOUND, "Not found".to_owned()))
    }
  }
}
