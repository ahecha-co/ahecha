use std::fmt::{Result, Write};

mod arrays;
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
