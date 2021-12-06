use ahecha_macro::*;

mod ahecha {
  pub use ahecha_html as html;
}

#[page(document = "crate::app::document::Document")]
pub fn CountPage(count: u32) -> ahecha::html::Node {
  html! {
    <div>Test page<span>{ count }</span></div>
  }
}
