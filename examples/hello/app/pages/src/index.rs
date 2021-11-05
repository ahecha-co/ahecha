use etagere::*;

use components::ExampleTag;

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
          <div>"Hello world"</div>
          // It might be possible to extract the metadata information from the component from within the macro?
          // This will be the ideal syntax
          <ExampleTag>"This is an example tag"</ExampleTag>
        </body>
      </html>
    }
  }
}
