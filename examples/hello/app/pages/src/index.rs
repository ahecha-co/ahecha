use etagere::*;

use components::PostComponent;

#[page]
#[derive(Default)]
pub struct IndexPage {
  // #[param] will be set from the url path, example /index/<param> == index/value
// example_param: &'static str,
// #[query_param] will be set from the url path, example /index/?<param> == index?param=value
// example_query_param: u8,
// #[prop]
// user: User,
// #[state(default = 0)]
// count: usize,
}

impl<'a> view::CustomElement<'a> for IndexPage<'a> {
  fn render(&self) -> view::Html<'a> {
    html! {
      <html lang="en">
        <body>
          <h1>"Etagere blog example"</h1>
          <PostComponent title="Hello, world!" body="This is the first post." image="https://cataas.com/cat" />
        </body>
      </html>
    }
  }
}
