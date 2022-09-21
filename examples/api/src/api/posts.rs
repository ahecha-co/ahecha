use axum::Json;

#[::ahecha::route(GET, "/")]
pub async fn posts() -> Json<String> {
  Json(
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
      .to_string(),
  )
}
