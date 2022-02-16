use std::fmt::{Result, Write};

use crate::html::Doctype;

use super::RenderString;

impl RenderString for Doctype {
  fn render_into<W: Write>(self, writer: &mut W) -> Result {
    match self {
      Doctype::Html5 => {
        writer.write_str("<!doctype html>")?;
      }
    }

    Ok(())
  }
}
