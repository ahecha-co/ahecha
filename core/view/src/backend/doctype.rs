use std::fmt::{Result, Write};

use crate::backend::render::Render;

pub enum HtmlDoctype {
  Html5,
}

impl Render for HtmlDoctype {
  fn render_into<W: Write>(self, writer: &mut W) -> Result {
    write!(writer, "<!doctype html>")
  }
}

impl From<HtmlDoctype> for String {
  fn from(element: HtmlDoctype) -> Self {
    element.render()
  }
}
