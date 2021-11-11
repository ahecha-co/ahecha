use std::fmt::{Result, Write};

use ita_tuple_list::TupleList;

use crate::backend::render::Render;

impl<Head, Tail> Render for (Head, Tail)
where
  Head: Render,
  Tail: Render + TupleList,
{
  fn render_into<W: Write>(self, writer: &mut W) -> Result {
    self.0.render_into(writer)?;
    self.1.render_into(writer)
  }
}
