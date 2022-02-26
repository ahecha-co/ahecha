use axum_core::{
  body,
  response::{IntoResponse, Response},
};
use http::StatusCode;
use http_body::Full;

use crate::{Node, RenderString};

impl IntoResponse for Node {
  fn into_response(self) -> Response {
    let body = body::boxed(Full::from(self.render()));

    Response::builder()
      .header("Content-Type", "text/html")
      .status(StatusCode::OK)
      .body(body)
      .unwrap()
  }
}
