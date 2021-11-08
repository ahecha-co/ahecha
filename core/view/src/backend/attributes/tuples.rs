use std::fmt::{Display, Result, Write};

use super::RenderAttributes;

impl RenderAttributes for () {
  fn render_attributes_into<W: Write>(&self, _writer: &mut W) -> Result {
    Ok(())
  }
}

// TODO: write a macro_rule to implement for more tuples
impl<A: Display> RenderAttributes for ((&str, A),) {
  fn render_attributes_into<W: Write>(&self, writer: &mut W) -> Result {
    self.0.render_attributes_into(writer)
  }
}

impl<A: Display, B: Display> RenderAttributes for ((&str, A), (&str, B)) {
  fn render_attributes_into<W: Write>(&self, writer: &mut W) -> Result {
    self.0.render_attributes_into(writer)?;
    self.1.render_attributes_into(writer)?;
    Ok(())
  }
}

impl<A: Display, B: Display, C: Display> RenderAttributes for ((&str, A), (&str, B), (&str, C)) {
  fn render_attributes_into<W: Write>(&self, writer: &mut W) -> Result {
    self.0.render_attributes_into(writer)?;
    self.1.render_attributes_into(writer)?;
    self.2.render_attributes_into(writer)?;
    Ok(())
  }
}

impl<A: Display, B: Display, C: Display, D: Display> RenderAttributes
  for ((&str, A), (&str, B), (&str, C), (&str, D))
{
  fn render_attributes_into<W: Write>(&self, writer: &mut W) -> Result {
    self.0.render_attributes_into(writer)?;
    self.1.render_attributes_into(writer)?;
    self.2.render_attributes_into(writer)?;
    self.3.render_attributes_into(writer)?;
    Ok(())
  }
}
