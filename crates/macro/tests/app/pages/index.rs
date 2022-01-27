use ahecha::prelude::*;

#[page(document = "crate::app::document::Document", title = "Index")]
pub fn IndexPage() -> ahecha::html::Node {
  html! {
    <div>Index page</div>
  }
}
