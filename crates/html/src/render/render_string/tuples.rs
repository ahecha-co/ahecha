use std::fmt::{Result, Write};

use ahecha_tuple_list::TupleList;

use crate::render::RenderString;

impl<Head, Tail> RenderString for (Head, Tail)
where
  Head: RenderString,
  Tail: RenderString + TupleList,
{
  fn render_into<W: Write>(self, writer: &mut W) -> Result {
    self.0.render_into(writer)?;
    self.1.render_into(writer)
  }
}
