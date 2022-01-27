use ahecha::prelude::*;

#[page(document = "crate::app::document::Document")]
pub fn CountPage(count: u32) -> ahecha::html::Node {
  html! {
    <div>Test page<span>{ count }</span></div>
  }
}
