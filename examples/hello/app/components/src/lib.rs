use etagere::*;

#[component]
pub struct ExampleTag;

impl view::Component for ExampleTag {
  fn render(&self) -> view::Html {
    html! {
      <div>"Example content"</div>
    }
  }
}
