use app::page;

mod app;

mod ahecha {
  pub use ahecha_view as view;
}

#[test]
fn test_generate_route() {
  assert_eq!(page::index::IndexPage::uri(), "/");
  assert_eq!(page::__count__::CountPage::uri(5), "/5");
}

#[test]
fn test_index_page_request() {
  let response = page::index::IndexPage::mount();
  assert_eq!(response, "<div>Index page</div>");
}

#[test]
fn test_test_page_request() {
  let response = page::__count__::CountPage::mount(5);
  assert_eq!(response, "<div>Test page<span>5</span></div>");
}
