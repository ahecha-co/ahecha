use crate::{html::Node, RenderString};

impl RenderString for Node {
  fn render_into<W: std::fmt::Write>(self, writer: &mut W) -> std::fmt::Result {
    match self {
      Self::Comment(elements) => {
        write!(writer, "<!--")?;
        elements.render_into(writer)?;
        write!(writer, "-->")?;
      }
      Self::CustomElement => todo!("CustomElement"),
      Self::Document(doctype, elements) => {
        doctype.render_into(writer)?;
        elements.render_into(writer)?;
      }
      Self::Element(element) => element.render_into(writer)?,
      Self::Fragment(elements) => elements.render_into(writer)?,
      Self::None => (),
      Self::LiveView(partial) => partial.render_into(writer)?,
      Self::Raw(text) => write!(writer, "{}", text)?,
      // FIXME: LOL
      Self::Redirect(_, location) => write!(
        writer,
        "<script>window.location.href = '{}';</script>",
        location
      )?,
      Self::Text(text) => text.render_into(writer)?,
    }

    Ok(())
  }
}
