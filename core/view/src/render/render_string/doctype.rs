use std::fmt::{Result, Write};

use crate::HtmlDoctype;

use super::RenderString;

impl<T> RenderString for HtmlDoctype<T>
where
  T: RenderString,
{
  fn render_into<W: Write>(self, writer: &mut W) -> Result {
    match self {
      HtmlDoctype::Html5(children) => {
        writer.write_str("<!doctype html>")?;
        children.render_into(writer)
      }
    }
  }
}

impl<T> From<HtmlDoctype<T>> for String
where
  T: RenderString,
{
  fn from(element: HtmlDoctype<T>) -> Self {
    element.render()
  }
}
