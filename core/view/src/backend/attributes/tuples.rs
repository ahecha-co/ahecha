use std::fmt::{Result, Write};

use super::RenderAttributes;

impl RenderAttributes for () {
  fn render_attributes_into<W: Write>(&self, _writer: &mut W) -> Result {
    Ok(())
  }
}

// TODO: write a macro_rule to implement for more tuples
impl RenderAttributes for ((&str, &str),) {
  fn render_attributes_into<W: Write>(&self, writer: &mut W) -> Result {
    [self.0].render_attributes_into(writer)
  }
}

impl RenderAttributes for ((&str, &str), (&str, &str)) {
  fn render_attributes_into<W: Write>(&self, writer: &mut W) -> Result {
    [self.0, self.1].render_attributes_into(writer)
  }
}
