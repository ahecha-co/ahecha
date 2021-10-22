#![feature(type_alias_impl_trait)]

use std::{
  borrow::Cow,
  collections::HashMap,
  fmt::{Result, Write},
};

pub use html_tag::HtmlTag;
pub use to_html::ToHtml;

pub use crate::html_escaping::escape_html;

mod html_escaping;
mod html_tag;
mod to_html;

pub type Html = String;

pub trait Component {
  fn render(&self) -> Html;
}

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
