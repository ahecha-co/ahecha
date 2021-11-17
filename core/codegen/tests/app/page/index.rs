use ahecha_codegen::*;

mod ahecha {
  pub use ahecha_view as view;
}

#[page(document = "crate::app::document::Document")]
pub fn IndexPage() {
  html! {
    <div>Index page</div>
  }
}
