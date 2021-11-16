use ahecha::view::Render;
use ahecha_codegen::*;

mod ahecha {
  pub use ahecha_view as view;
}

#[page]
pub fn CountPage(count: u32) {
  html! {
    <div>Test page<span>{ count }</span></div>
  }
}
