use axum_core::{
  body,
  response::{IntoResponse, Response},
};
use http::{
  header::{CONTENT_TYPE, LOCATION},
  StatusCode,
};
use http_body::{Empty, Full};

use crate::{Node, RenderString};

impl IntoResponse for Node {
  fn into_response(self) -> Response {
    match self {
      Node::Redirect(status_code, location) => Response::builder()
        .header(LOCATION, location.to_string())
        .status(status_code)
        .body(body::boxed(Empty::new()))
        .unwrap(),
      _ => Response::builder()
        .header(CONTENT_TYPE, "text/html")
        .status(StatusCode::OK)
        .body(body::boxed(Full::from(self.render())))
        .unwrap(),
    }
  }
}
