use std::fmt::{Result, Write};

use super::render::Render;

pub struct BodyElement<T>
where
  T: Render,
{
  pub root: T,
}

impl<T> Render for BodyElement<T>
where
  T: Render,
{
  fn render_into<W: Write>(self, writer: &mut W) -> Result {
    self.root.render_into(writer)
  }
}
