use axum_core::{
  body::{self, boxed},
  response::{IntoResponse, Response},
};
use http::{header::LOCATION, HeaderValue, StatusCode};
use http_body::{Empty, Full};

use crate::{Node, RenderString};

impl IntoResponse for Node {
  fn into_response(self) -> Response {
    match self.get_redirect() {
      Some((status_code, location)) => {
        let mut res = Response::new(boxed(Empty::new()));
        *res.status_mut() = status_code;
        res.headers_mut().insert(
          LOCATION,
          HeaderValue::try_from(location.to_string()).expect("URI isn't a valid header value"),
        );
        res
      }
      None => {
        let body = body::boxed(Full::from(self.render()));

        Response::builder()
          .header("Content-Type", "text/html")
          .status(StatusCode::OK)
          .body(body)
          .unwrap()
      }
    }
  }
}
