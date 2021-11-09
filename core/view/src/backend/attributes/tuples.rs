use std::fmt::{Display, Result, Write};
use tuple_list::TupleList;

use crate::escape_html;

use super::RenderAttributes;

impl RenderAttributes for () {
  fn render_attributes_into<W: Write>(&self, _writer: &mut W) -> Result {
    Ok(())
  }
}

impl<A> RenderAttributes for (&str, A)
where
  A: Display,
{
  fn render_attributes_into<W: Write>(&self, writer: &mut W) -> Result {
    write!(writer, " {}=\"", self.0)?;
    escape_html(&self.1, writer)?;
    write!(writer, "\"")
  }
}

impl<Head, Tail> RenderAttributes for (Head, Tail)
where
  Head: RenderAttributes,
  Tail: RenderAttributes + TupleList,
{
  fn render_attributes_into<W: Write>(&self, writer: &mut W) -> Result {
    self.0.render_attributes_into(writer)?;
    self.1.render_attributes_into(writer)
  }
}
