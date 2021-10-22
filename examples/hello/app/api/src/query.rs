use etagere::route;

pub mod __id__;

// Don't know yet about these http verbs CONNECT, OPTIONS, TRACE

// Example of a query it will be mapped to GET /api/query
// GET, HEAD are considered queries
#[route]
pub fn get() -> &'static str {
  // Response::Ok(ResponseFormat::Json(MessageResponse { message: "example" }))
  "hello world"
}
