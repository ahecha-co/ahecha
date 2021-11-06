use std::borrow::Cow;

use etagere::*;
use models::*;

#[derive(Default)]
pub struct PostComponent<'a> {
  post: Post<'a>,
}

impl<'a> view::CustomElement<'a> for PostComponent<'a> {
  type Attributes = Post<'a>;

  fn create(&mut self, attributes: Self::Attributes, _: view::Html<'a>) {
    self.post = attributes;
  }

  fn attributes(&self) -> Self::Attributes {
    self.post.clone()
  }

  fn render(&self) -> view::Html<'a> {
    html! {
      <div class="px-4 py-5 my-5 text-center">
        <h1 class="display-5 fw-bold">{ self.post.title }</h1>
        <div class="col-lg-6 mx-auto">
          <p class="lead mb-4">{ self.post.body }</p>
          <div class="d-grid gap-2 d-sm-flex justify-content-sm-center">
            <img src={ self.post.image } />
          </div>
        </div>
      </div>
    }
  }
}
