use etagere::*;

#[derive(Page)]
pub struct Page403;

impl Page403 {
    type Html: ToHtml;
  pub fn render(&self, document: &mut Document) -> Box<dyn ToHtml> {
    document.render(html! {})
  }
}
