// This macro cleans the target directory before each compilation,
// to prevent orphan files that could cause to generate missing routes.
::ahecha::monkey_path_clean!();

mod api;

#[cfg(not(target_arch = "wasm32"))]
async fn server(router: axum::Router) -> Result<(), String> {
  axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(router.into_make_service())
    .await
    .unwrap();
  Ok(())
}

#[tokio::main]
async fn main() -> Result<(), String> {
  ::ahecha::router!()
}
