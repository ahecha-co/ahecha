use ahecha::view::Render;
use ahecha_codegen::*;

mod ahecha {
  pub use ahecha_view as view;
}

#[page]
pub fn TestPage() {
  html! {
    <div>Test page</div>
  }
}
