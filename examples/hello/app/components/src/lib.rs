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

#[derive(Default)]
pub struct PostComponent<'a> {
  attributes: Vec<(&'a str, &'a str)>,
}

impl<'a> PostComponent<'a> {
  pub fn get_title(&self) -> &str {
    self
      .attributes
      .iter()
      .find(|(key, _)| *key == "title")
      .unwrap()
      .1
  }

  pub fn get_body(&self) -> &str {
    self
      .attributes
      .iter()
      .find(|(key, _)| *key == "body")
      .unwrap()
      .1
  }

  pub fn get_image(&self) -> &str {
    self
      .attributes
      .iter()
      .find(|(key, _)| *key == "image")
      .unwrap()
      .1
  }
}

impl<'a> view::CustomElement<'a> for PostComponent<'a> {
  fn create(&mut self, attributes: Vec<(&'a str, &'a str)>, _: view::Html<'a>) {
    self.attributes = attributes;
  }

  fn attributes(&self) -> Vec<(&'a str, &'a str)> {
    self.attributes.clone()
  }

  fn render(&self) -> view::Html<'a> {
    html! {
      <div class="px-4 py-5 my-5 text-center">
        <h1 class="display-5 fw-bold">{ view::Node::Text(&'a self.get_title()) }</h1>
        <div class="col-lg-6 mx-auto">
          <p class="lead mb-4">{ view::Node::Text(&'a self.get_body()) }</p>
          <div class="d-grid gap-2 d-sm-flex justify-content-sm-center">
            <img src={ self.get_image() } />
          </div>
        </div>
      </div>
    }
  }
}
