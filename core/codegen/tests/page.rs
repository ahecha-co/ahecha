use ahecha_codegen::html;
use ahecha_view::Render;
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
  let response = page::index::IndexPage::handler().render();
  assert_eq!(
    response,
    "<html><head><title>Index</title></head><body><div>Index page</div></body></html>"
  );
}

#[test]
fn test_test_page_request() {
  let response = page::__count__::CountPage::handler(5).render();
  assert_eq!(
    response,
    "<html><head><title>Document title</title></head><body><div>Test page<span>5</span></div></body></html>"
  );
}

#[test]
fn test_page_as_partial() {
  use page::__count__::CountPage;

  let res = html!(
    <div>
      <CountPage count=5 />
    </div>
  );

  assert_eq!(
    res.render(),
    "<div><div>Test page<span>5</span></div></div>"
  );
}
