#[cfg(feature = "extra")]
mod test {
  use ahecha::{
    html::{Component, Node, RenderString},
    prelude::{html, Page},
  };

  #[derive(Page)]
  #[route("/simple_page")]
  struct SimplePage<'a> {
    partial: &'a ahecha::html::partials::PartialBuilder,
  }

  impl<'a> Component for SimplePage<'a> {
    fn view(&self) -> Node {
      let _ = self.partial;
      html!(<div>Hello world</div>)
    }
  }

  #[test]
  fn simpl_page() {
    assert_eq!(
      "<div>Hello world</div>",
      SimplePage {
        partial: &ahecha::html::partials::PartialBuilder::new("/"),
      }
      .view()
      .render()
    );

    assert_eq!("/simple_page", SimplePagePath {}.to_string())
  }
}
