use ahecha::view::Render;
use ahecha_codegen::*;

mod ahecha {
  pub use ahecha_view as view;
}

#[page]
pub fn IndexPage() {
  html! {
    <div>Index page</div>
  }
}
