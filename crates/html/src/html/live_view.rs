use crate::{Attributes, Children, RenderString};

#[derive(Debug, Clone)]
pub struct LiveView {
  pub attributes: Attributes,
  pub children: Children,
  pub id: String,
}

impl RenderString for LiveView {
  fn render_into<W: std::fmt::Write>(self, writer: &mut W) -> std::fmt::Result {
    write!(writer, r#"<live-view"#)?;
    self.attributes.render_into(writer)?;
    write!(writer, ">")?;
    self.children.render_into(writer)?;
    write!(writer, "</live-view>")?;

    Ok(())
  }
}
