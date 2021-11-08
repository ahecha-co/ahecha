use std::fmt::{Result, Write};

use crate::escape_html;

use super::RenderAttributes;

// TODO: write a macro_rule to implement for more array sizes
impl RenderAttributes for [(&str, &str); 1] {
  fn render_attributes_into<W: Write>(&self, writer: &mut W) -> Result {
    self.iter().try_for_each(|(key, value)| {
      write!(writer, " {}=\"", key)?;
      escape_html(value, writer)?;
      write!(writer, "\"")
    })
  }
}

impl RenderAttributes for [(&str, &str); 2] {
  fn render_attributes_into<W: Write>(&self, writer: &mut W) -> Result {
    self.iter().try_for_each(|(key, value)| {
      write!(writer, " {}=\"", key)?;
      escape_html(value, writer)?;
      write!(writer, "\"")
    })
  }
}
