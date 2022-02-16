mod attribute;
mod node;
mod tag;

pub use self::{node::Node, tag::Tag};

pub trait ToHtmlString: Sized {
  fn render_into<W: std::fmt::Write>(self, buffer: &mut W) -> std::fmt::Result;

  fn render(self) -> String {
    let mut buffer = String::new();
    self.render_into(&mut buffer).unwrap();
    buffer
  }
}

impl<T: ToHtmlString> ToHtmlString for Vec<T> {
  fn render_into<W: std::fmt::Write>(self, buffer: &mut W) -> std::fmt::Result {
    for elem in self {
      elem.render_into(buffer)?;
    }
    Ok(())
  }
}

pub trait IntoHtml {
  fn into_node(&self) -> Node;
}
