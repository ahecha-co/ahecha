use std::fmt::{Result, Write};

use crate::escape_html;

use super::RenderAttributes;

impl<const DINENSION: usize> RenderAttributes for [(&str, &str); DINENSION] {
  fn render_attributes_into<W: Write>(&self, writer: &mut W) -> Result {
    self.iter().try_for_each(|(key, value)| {
      write!(writer, " {}=\"", key)?;
      escape_html(value, writer)?;
      write!(writer, "\"")
    })
  }
}
