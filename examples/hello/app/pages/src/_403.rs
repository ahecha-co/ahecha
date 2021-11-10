use ita::*;

#[derive(Page)]
pub struct Page403;

impl Page403 {
  type Html;
  pub fn render(&self, document: &mut Document) -> Box<dyn ToHtml> {
    document.render(html! {})
  }
}
