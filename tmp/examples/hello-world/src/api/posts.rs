#[::router::route]
pub async fn posts() -> impl axum::response::IntoResponse {
  r#"[
  {
    "title": "Post #1",
    "content": "Content #1",
  },
  {
    "title": "Post #2",
    "content": "Content #2",
  }
  ]"#
}
