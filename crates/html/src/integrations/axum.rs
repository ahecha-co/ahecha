use ::axum::{
  body,
  http::StatusCode,
  response::{IntoResponse, Response},
};

use crate::{Node, RenderString};

impl IntoResponse for Node {
  fn into_response(self) -> Response {
    let body = body::boxed(body::Full::from(self.render()));

    Response::builder()
      .header("Content-Type", "text/html")
      .status(StatusCode::OK)
      .body(body)
      .unwrap()
  }
}
