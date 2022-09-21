// This macro cleans the target directory before each compilation,
// to prevent orphan files that could cause to generate missing routes.
::ahecha::monkey_path_clean!();

mod api;

#[cfg(not(target_arch = "wasm32"))]
async fn server(router: axum::Router) -> Result<(), String> {
  async fn handle_error(_err: std::io::Error) -> impl axum::response::IntoResponse {
    (
      axum::http::StatusCode::INTERNAL_SERVER_ERROR,
      "Something went wrong...",
    )
  }

  axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(
      router
        .fallback(
          axum::routing::get_service(tower_http::services::ServeDir::new("./public"))
            .handle_error(handle_error),
        )
        .into_make_service(),
    )
    .await
    .unwrap();

  Ok(())
}

#[tokio::main]
async fn main() -> Result<(), String> {
  ::ahecha::router!()
}
