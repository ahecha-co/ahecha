#![feature(type_alias_impl_trait)]

use std::{
  borrow::Cow,
  collections::HashMap,
  fmt::{Result, Write},
};

pub use html_escaping::escape_html;

mod backend;
mod html_escaping;

pub use backend::render::Render;

pub type Attributes<'a> = Option<HashMap<&'a str, Cow<'a, str>>>;

pub fn write_attributes<'a, W: Write>(maybe_attributes: Attributes<'a>, writer: &mut W) -> Result {
  match maybe_attributes {
    None => Ok(()),
    Some(mut attributes) => {
      for (key, value) in attributes.drain() {
        write!(writer, " {}=\"", key)?;
        escape_html(&value, writer)?;
        write!(writer, "\"")?;
      }
      Ok(())
    }
  }
}
