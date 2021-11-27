use ahecha_macro::*;

mod ahecha {
  pub use ahecha_html as view;
}

#[page(document = "crate::app::document::Document")]
pub fn CountPage(count: u32) {
  html! {
    <div>Test page<span>{ count }</span></div>
  }
}
