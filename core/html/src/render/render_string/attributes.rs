// use ahecha_tuple_list::TupleList;
use std::{
  collections::HashMap,
  fmt::{Result, Write},
};

use crate::{escape_html, html::AttributeValue};

pub trait RenderAttributeValue {
  fn to_attribute_value(&self) -> String;
}

impl RenderAttributeValue for Option<String> {
  fn to_attribute_value(&self) -> String {
    match self {
      Some(s) => s.to_owned(),
      None => "".to_string(),
    }
  }
}

impl RenderAttributeValue for Option<&str> {
  fn to_attribute_value(&self) -> String {
    match self {
      Some(s) => s.to_string(),
      None => "".to_string(),
    }
  }
}

macro_rules! impl_attribute_value {
  ($($t:ty),*) => {
    $(impl RenderAttributeValue for $t {
      fn to_attribute_value(&self) -> String {
        self.to_string()
      }
    })*
  };
}

impl RenderAttributeValue for AttributeValue {
  fn to_attribute_value(&self) -> String {
    match self {
      Self::String(value) => value.to_owned(),
      Self::Bool(value) => value.to_string(),
    }
  }
}

impl_attribute_value!(&str, &String, String, bool, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);

impl RenderAttributes for HashMap<String, AttributeValue> {
  fn render_attributes_into<W: Write>(self, writer: &mut W) -> Result {
    for (k, v) in self.iter() {
      write!(writer, " {}=\"", k)?;
      escape_html(&v.to_attribute_value(), writer)?;
      write!(writer, "\"")?;
    }

    Ok(())
  }
}

impl RenderAttributes for Vec<(String, AttributeValue)> {
  fn render_attributes_into<W: Write>(self, writer: &mut W) -> Result {
    // self.sort_by(|a, b| a.0.partial_cmp(&b.0));
    for (k, v) in self.iter() {
      write!(writer, " {}=\"", k)?;
      escape_html(&v.to_attribute_value(), writer)?;
      write!(writer, "\"")?;
    }

    Ok(())
  }
}

pub trait RenderAttributes: Sized {
  fn render_attributes_into<W: Write>(self, writer: &mut W) -> Result;
}

impl RenderAttributes for () {
  fn render_attributes_into<W: Write>(self, _writer: &mut W) -> Result {
    Ok(())
  }
}

impl<A> RenderAttributes for (&str, A)
where
  A: RenderAttributeValue,
{
  fn render_attributes_into<W: Write>(self, writer: &mut W) -> Result {
    write!(writer, " {}=\"", self.0)?;
    escape_html(&self.1.to_attribute_value(), writer)?;
    write!(writer, "\"")
  }
}

// impl<A, Tail> RenderAttributes for ((&str, A), Tail)
// where
//   A: RenderAttributeValue,
//   Tail: RenderAttributes + TupleList,
// {
//   fn render_attributes_into<W: Write>(self, writer: &mut W) -> Result {
//     self.0.render_attributes_into(writer)?;
//     self.1.render_attributes_into(writer)
//   }
// }
