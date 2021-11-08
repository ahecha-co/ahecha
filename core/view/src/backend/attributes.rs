use std::fmt::{Display, Result, Write};

use crate::escape_html;

mod arrays;
// mod numbers;
mod tuples;
mod vectors;

pub trait RenderAttributes: Sized {
  fn render_attributes_into<W: Write>(&self, writer: &mut W) -> Result;
}

impl<A> RenderAttributes for Option<A>
where
  A: RenderAttributes,
{
  fn render_attributes_into<W: Write>(&self, writer: &mut W) -> Result {
    if let Some(a) = self {
      a.render_attributes_into(writer)?;
    }
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
