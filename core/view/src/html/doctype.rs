use std::fmt::{Result, Write};

use crate::html::render::Render;

pub enum HtmlDoctype<T> {
  Html5(T),
}

impl<T> Render for HtmlDoctype<T>
where
  T: Render,
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
  T: Render,
{
  fn from(element: HtmlDoctype<T>) -> Self {
    element.render()
  }
}
