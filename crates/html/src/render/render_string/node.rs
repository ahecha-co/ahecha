use crate::{html::Node, RenderString};

impl RenderString for Node {
  fn render_into<W: std::fmt::Write>(self, writer: &mut W) -> std::fmt::Result {
    match self {
      Self::CustomElement => todo!("CustomElement"),
      Self::Document(doctype, elements) => {
        doctype.render_into(writer)?;
        elements.render_into(writer)?;
      }
      Self::Element(element) => element.render_into(writer)?,
      Self::Fragment(elements) => elements.render_into(writer)?,
      Self::None => (),
      Self::Text(text) => text.render_into(writer)?,
    }

    Ok(())
  }
}
