use crate::{write_attributes, Attributes, ToHtml};

use std::fmt::{Result, Write};

pub struct SimpleElement<'a, T: ToHtml> {
  pub tag_name: &'a str,
  pub attributes: Attributes<'a>,
  pub children: Option<T>,
}

impl<T: ToHtml> ToHtml for SimpleElement<'_, T> {
  fn html_into<W: Write>(self, writer: &mut W) -> Result {
    match self.children {
      None => {
        write!(writer, "<{}", self.tag_name)?;
        write_attributes(self.attributes, writer)?;
        write!(writer, "/>")
      }
      Some(renderable) => {
        write!(writer, "<{}", self.tag_name)?;
        write_attributes(self.attributes, writer)?;
        write!(writer, ">")?;
        renderable.html_into(writer)?;
        write!(writer, "</{}>", self.tag_name)
      }
    }
  }
}
