// This macro cleans the target directory before each compilation,
// to prevent orphan files that could cause to generate missing routes.
::ahecha::monkey_path_clean!();

mod api;
mod pages;

#[cfg(not(target_arch = "wasm32"))]
type Db = std::sync::Arc<std::sync::RwLock<std::collections::HashMap<uuid::Uuid, api::Todo>>>;

#[cfg(not(target_arch = "wasm32"))]
async fn server(router: axum::Router) -> Result<(), String> {
  async fn handle_error(_err: std::io::Error) -> impl axum::response::IntoResponse {
    (
      axum::http::StatusCode::INTERNAL_SERVER_ERROR,
      "Something went wrong...",
    )
  }

  let db = Db::default();

  axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
    .serve(
      router
        .fallback(
          axum::routing::get_service(tower_http::services::ServeDir::new("./public"))
            .handle_error(handle_error),
        )
        .layer(axum::extract::Extension(db))
        .into_make_service(),
    )
    .await
    .unwrap();

  Ok(())
}

#[cfg(target_arch = "wasm32")]
fn client() {
  wasm_logger::init(wasm_logger::Config::new(tracing::log::Level::Debug));
  tracing_wasm::set_as_global_default();
  dioxus::web::launch(pages::Index);
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<(), String> {
  ::ahecha::router!()
}

#[cfg(target_arch = "wasm32")]
fn main() {
  client()
}
