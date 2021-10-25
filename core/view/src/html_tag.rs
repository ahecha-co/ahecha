use crate::{write_attributes, Attributes, ToHtml};

use std::fmt::{Result, Write};

#[derive(Clone)]
pub struct HtmlTag<'a, T: ToHtml> {
  pub tag_name: &'a str,
  pub attributes: Attributes<'a>,
  pub children: Vec<T>,
}

impl<T: ToHtml> ToHtml for HtmlTag<'_, T> {
  fn html_into<W: Write>(&self, writer: &mut W) -> Result {
    if self.children.is_empty() {
      write!(writer, "<{}", self.tag_name)?;
      write_attributes(self.attributes.clone(), writer)?;
      write!(writer, "/>")
    } else {
      write!(writer, "<{}", self.tag_name)?;
      write_attributes(self.attributes.clone(), writer)?;
      write!(writer, ">")?;
      self.children.iter().try_for_each(|c| c.html_into(writer))?;
      write!(writer, "</{}>", self.tag_name)
    }
  }
}

impl<T: ToHtml> From<HtmlTag<'_, T>> for String {
  fn from(item: HtmlTag<T>) -> Self {
    item.to_html()
  }
}

impl<T: ToHtml> From<&HtmlTag<'_, T>> for String {
  fn from(item: &HtmlTag<T>) -> Self {
    item.to_html()
  }
}
