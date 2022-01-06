mod app;

use ahecha::prelude::*;
use app::pages;

#[test]
fn test_generate_route() {
  assert_eq!(pages::index::IndexPage::uri(), "/");
  assert_eq!(pages::__count__::CountPage::uri(5), "/5");
}

#[test]
fn test_page_as_partial() {
  use pages::__count__::CountPage;

  let res = html!(
    <div>
      <CountPage count=5 />
    </div>
  )
  .render();

  assert_eq!(res, "<div><div>Test page<span>5</span></div></div>");
}

#[cfg(feature = "frontend")]
mod frontend {
  use super::*;

  #[test]
  fn test_index_page_partial() {
    let response: String = pages::index::IndexPage::view().render();
    assert_eq!(response, "<div>Index page</div>");
  }

  #[test]
  fn test_test_page_partial() {
    let response =
      pages::__count__::CountPage::view(pages::__count__::CountPage::ViewParams { count: 5 })
        .render();
    assert_eq!(response, "<div>Test page<span>5</span></div>");
  }
}

#[cfg(feature = "backend")]
mod backend {
  use super::*;

  #[test]
  fn test_index_page_request() {
    let response: String = pages::index::IndexPage::handler().render();
    assert_eq!(
      response,
      "<html><head><title>Index</title></head><body><div>Index page</div></body></html>"
    );
  }

  #[test]
  fn test_test_page_request() {
    let response = pages::__count__::CountPage::handler(5).render();
    assert_eq!(
      response,
      "<html><head><title>Document title</title></head><body><div>Test page<span>5</span></div></body></html>"
    );
  }
}
