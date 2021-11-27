use ahecha_macro::*;

mod ahecha {
  pub use ahecha_html as html;
}

#[page(document = "crate::app::document::Document", title = "Index")]
pub fn IndexPage() {
  html! {
    <div>Index page</div>
  }
}
