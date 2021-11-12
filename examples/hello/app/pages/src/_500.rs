use ahecha::*;

#[derive(Page)]
pub struct Page500;

impl IndexPage {
  type Html;
  pub fn render(&self, document: &mut Document) -> Box<dyn ToHtml> {
    document.render(html! {})
  }
}
