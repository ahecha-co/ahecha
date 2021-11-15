use app::page;
use tokio;

mod app;

mod ahecha {
  pub use ahecha_view as view;
}

#[test]
fn test_generate_route() {
  assert_eq!(page::index::IndexPage::route(), "/");
  assert_eq!(page::test::TestPage::route(), "/test");
}

#[tokio::test]
async fn test_index_page_request() {
  let response = warp::test::request()
    .method("GET")
    .path("/")
    .reply(&page::index::IndexPage::mount())
    .await;
  assert_eq!(response.status(), 200);
  assert_eq!(response.body(), "<div>Index page</div>");
}

#[tokio::test]
async fn test_test_page_request() {
  let response = warp::test::request()
    .method("GET")
    .path("/test")
    .reply(&page::test::TestPage::mount())
    .await;
  assert_eq!(response.status(), 200);
  assert_eq!(response.body(), "<div>Test page</div>");
}
