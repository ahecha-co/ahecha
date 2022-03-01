use crate::{Children, RenderString};

impl RenderString for Children {
  fn render_into<W: std::fmt::Write>(self, writer: &mut W) -> std::fmt::Result {
    self.children.render_into(writer)
  }
}
