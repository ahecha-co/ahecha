use std::fmt::{Result, Write};
use tuple_list::TupleList;

use crate::escape_html;

pub trait ToAttributeValue {
  fn to_attribute_value(&self) -> String;
}

impl ToAttributeValue for Option<&str> {
  fn to_attribute_value(&self) -> String {
    match self {
      Some(s) => s.to_string(),
      None => "".to_string(),
    }
  }
}

macro_rules! impl_attribute_value {
  ($($t:ty),*) => {
    $(impl ToAttributeValue for $t {
      fn to_attribute_value(&self) -> String {
        self.to_string()
      }
    })*
  };
}

impl_attribute_value!(&str, String, bool, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);

pub trait RenderAttributes: Sized {
  fn render_attributes_into<W: Write>(&self, writer: &mut W) -> Result;
}

impl RenderAttributes for () {
  fn render_attributes_into<W: Write>(&self, _writer: &mut W) -> Result {
    Ok(())
  }
}

impl<A> RenderAttributes for (&str, A)
where
  A: ToAttributeValue,
{
  fn render_attributes_into<W: Write>(&self, writer: &mut W) -> Result {
    write!(writer, " {}=\"", self.0)?;
    escape_html(&self.1.to_attribute_value(), writer)?;
    write!(writer, "\"")
  }
}

impl<A, Tail> RenderAttributes for ((&str, A), Tail)
where
  A: ToAttributeValue,
  Tail: RenderAttributes + TupleList,
{
  fn render_attributes_into<W: Write>(&self, writer: &mut W) -> Result {
    self.0.render_attributes_into(writer)?;
    self.1.render_attributes_into(writer)
  }
}
