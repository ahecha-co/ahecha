use etagere::*;

#[derive(Default)]
pub struct ExampleTag;

impl<'a> view::CustomElement<'a> for ExampleTag {
  fn create(&mut self, _attributes: Vec<(&'a str, &'a str)>, _children: view::Html<'a>) {}

  fn attributes(&self) -> Vec<(&'a str, &'a str)> {
    vec![]
  }

  fn render(&self) -> view::Html<'a> {
    html! {
      <div>"Example content"</div>
    }
  }
}
