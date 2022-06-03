use ahecha::html::{Component, IntoNode, Node, RenderString};
use axum::{
  extract::FromRequest,
  http::{
    header::{self},
    StatusCode,
  },
  response::{Html, IntoResponse, Response},
  Json,
};
use serde::Serialize;

pub trait Layout {
  fn view<B>(self, body: B) -> Node
  where
    B: Into<Node>;
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
pub enum Format {
  Html,
  HtmlPartial,
  Json,
}

#[async_trait::async_trait]
impl<B> FromRequest<B> for Format
where
  B: Send,
{
  type Rejection = (StatusCode, String);

  async fn from_request(req: &mut axum::extract::RequestParts<B>) -> Result<Self, Self::Rejection> {
    if supports_content_type(req, "json") {
      Ok(Format::Json)
    } else if supports_content_type(req, "partial") {
      Ok(Format::HtmlPartial)
    } else if supports_content_type(req, "html") {
      Ok(Format::Html)
    } else {
      Err((StatusCode::NOT_FOUND, "Not found".to_owned()))
    }
  }
}

pub enum HtmlFormat {
  Html,
  HtmlPartial,
}

#[async_trait::async_trait]
impl<B> FromRequest<B> for HtmlFormat
where
  B: Send,
{
  type Rejection = (StatusCode, String);

  async fn from_request(req: &mut axum::extract::RequestParts<B>) -> Result<Self, Self::Rejection> {
    if supports_content_type(req, "partial") {
      Ok(HtmlFormat::HtmlPartial)
    } else if supports_content_type(req, "html") {
      Ok(HtmlFormat::Html)
    } else {
      Err((StatusCode::NOT_FOUND, "Not found".to_owned()))
    }
  }
}

pub struct HtmlPartialFormat;

#[async_trait::async_trait]
impl<B> FromRequest<B> for HtmlPartialFormat
where
  B: Send,
{
  type Rejection = (StatusCode, String);

  async fn from_request(req: &mut axum::extract::RequestParts<B>) -> Result<Self, Self::Rejection> {
    if supports_content_type(req, "partial") {
      Ok(HtmlPartialFormat)
    } else {
      Err((StatusCode::NOT_FOUND, "Not found".to_owned()))
    }
  }
}

pub trait IntoFormattedResponse {
  fn into_formatted_response(self) -> Response;
}

impl<L, C> IntoFormattedResponse for (Format, L, C)
where
  L: Layout,
  C: Component + Serialize,
{
  fn into_formatted_response(self) -> Response {
    let (format, layout, component) = self;

    match format {
      Format::Html => (
        component.status_code(),
        Html(
          layout
            .view(component.into_node())
            .normalize()
            .strip_slots()
            .render(),
        ),
      )
        .into_response(),
      Format::HtmlPartial => (
        component.status_code(),
        Html(component.into_node().strip_slots().render()),
      )
        .into_response(),
      Format::Json => (component.status_code(), Json(component)).into_response(),
    }
  }
}

impl<L, C> IntoFormattedResponse for (HtmlFormat, L, C)
where
  L: Layout,
  C: Component,
{
  fn into_formatted_response(self) -> Response {
    let (format, layout, component) = self;

    match format {
      HtmlFormat::Html => (
        component.status_code(),
        Html(
          layout
            .view(component.into_node())
            .normalize()
            .strip_slots()
            .render(),
        ),
      )
        .into_response(),
      HtmlFormat::HtmlPartial => (
        component.status_code(),
        Html(component.into_node().strip_slots().render()),
      )
        .into_response(),
    }
  }
}

impl<C> IntoFormattedResponse for (HtmlPartialFormat, C)
where
  C: Component,
{
  fn into_formatted_response(self) -> Response {
    let (_, component) = self;

    (
      component.status_code(),
      Html(component.into_node().strip_slots().render()),
    )
      .into_response()
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use ahecha::macros::html;
  use axum::body::HttpBody;

  async fn get_body(response: Response) -> String {
    String::from_utf8(response.data().await.unwrap().unwrap()[..].to_vec()).unwrap()
  }

  async fn get_html_response<L, C>(layout: L, component: C) -> String
  where
    L: Layout,
    C: Component + Serialize,
  {
    get_body((Format::Html, layout, component).into_formatted_response())
  }

  async fn get_html_partial_response<L, C>(layout: L, component: C) -> String
  where
    L: Layout,
    C: Component + Serialize,
  {
    get_body((Format::HtmlPartial, layout, component).into_formatted_response())
  }

  async fn get_json_response<L, C>(layout: L, component: C) -> String
  where
    L: Layout,
    C: Component + Serialize,
  {
    get_body((Format::Json, layout, component).into_formatted_response())
  }

  #[tokio::test]
  async fn simple_test() {
    #[derive(Deserialize, Serialize)]
    struct SimpleTest {
      title: &'static str,
    }

    impl Component for SimpleTest {
      fn view(self) -> Node {
        html!(<h1>{self.title}</h1>)
      }
    }

    struct TestLayout;

    impl Layout for TestLayout {
      fn view<B>(self, body: B) -> Node
      where
        B: Into<Node>,
      {
        html!(<div>{body}</div>).normalize()
      }
    }

    assert_eq!(
      "<div><h1>into_html</h1></div>",
      get_html_response(TestLayout, SimpleTest { title: "into_html" }).await
    );

    assert_eq!(
      r#"{"title":"into_json"}"#,
      get_json_response(TestLayout, SimpleTest { title: "into_json" }).await
    );

    assert_eq!(
      "<h1>into_partial</h1>",
      get_html_partial_response(
        TestLayout,
        SimpleTest {
          title: "into_partial"
        }
      )
      .await
    );
  }

  #[tokio::test]
  async fn slots_test() {
    #[derive(Deserialize, Serialize)]
    struct SlotTest {
      title: &'static str,
    }

    impl Component for SlotTest {
      fn view(self) -> Node {
        html!(
          <>
            <title slot="title">{self.title}</title>
            <span>World</span>
          </>
        )
      }
    }

    struct SimpleLayout;

    impl Layout for SimpleLayout {
      fn view<B>(self, body: B) -> Node
      where
        B: Into<Node>,
      {
        html!(<html><head><slot name="title"/></head><body>{body}</body></html>).normalize()
      }
    }

    assert_eq!(
      r#"<html><head><title slot="title">into_html</title></head><body><span>World</span></body></html>"#,
      get_html_response(SimpleLayout, SlotTest { title: "into_html" }).await
    );

    assert_eq!(
      r#"{"title":"into_json"}"#,
      get_json_response(SimpleLayout, SlotTest { title: "into_json" }).await
    );

    assert_eq!(
      r#"<title slot="title">into_partial</title><span>World</span>"#,
      get_html_partial_response(
        SimpleLayout,
        SlotTest {
          title: "into_partial",
        }
      )
      .await
    );
  }
}
