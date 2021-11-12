use ahecha::*;

#[derive(Page)]
pub struct Page404;

impl Page404 {
  type Html;
  pub fn render(&self, document: &mut Document) -> Box<dyn ToHtml> {
    document.render(html! {})
  }
}
