use crate::{html_escaping::escape_html, render::RenderString};
use std::fmt::{Result, Write};

impl RenderString for String {
  fn render_into<W: Write>(self, writer: &mut W) -> Result {
    escape_html(&self, writer)
  }
}

impl RenderString for &str {
  fn render_into<W: Write>(self, writer: &mut W) -> Result {
    escape_html(self, writer)
  }
}

impl RenderString for &&str {
  fn render_into<W: Write>(self, writer: &mut W) -> Result {
    escape_html(self, writer)
  }
}

impl RenderString for std::borrow::Cow<'_, str> {
  fn render_into<W: Write>(self, writer: &mut W) -> Result {
    escape_html(&self, writer)
  }
}

/// A raw (unencoded) html string
#[derive(Debug)]
pub struct Raw<'s>(&'s str);

impl<'s> From<&'s str> for Raw<'s> {
  fn from(s: &'s str) -> Self {
    Raw(s)
  }
}

/// A raw (unencoded) html string
impl<'s> RenderString for Raw<'s> {
  fn render_into<W: Write>(self, writer: &mut W) -> Result {
    write!(writer, "{}", self.0)
  }
}
