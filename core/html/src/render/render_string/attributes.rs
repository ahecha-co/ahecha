// use ahecha_tuple_list::TupleList;
use std::fmt::{Result, Write};

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

    $(impl From<$t> for AttributeValue {
      fn from(item: $t) -> AttributeValue {
        AttributeValue::String(item.to_string())
      }
    })*

    $(impl From<Option<$t>> for AttributeValue {
      fn from(item: Option<$t>) -> AttributeValue {
        match item {
          Some(item) => item.into(),
          None => AttributeValue::None,
        }
      }
    })*
  };
}

macro_rules! impl_attribute_value_with_into {
  ($($t:ty),*) => {
    $(impl_attribute_value!($t);)*

    $(impl From<AttributeValue> for $t {
      fn from(item: AttributeValue) -> $t {
        match item {
          AttributeValue::String(s) => s.parse().unwrap(),
          _ => unimplemented!(),
        }
      }
    })*
  };
}

impl RenderAttributeValue for AttributeValue {
  fn to_attribute_value(&self) -> String {
    match self {
      Self::Bool(value) => value.to_string(),
      Self::None => String::new(),
      Self::String(value) => value.to_owned(),
    }
  }
}

impl_attribute_value!(&&str, &str, &String, String, bool);
impl_attribute_value_with_into!(u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);

impl RenderAttributes for Vec<(String, AttributeValue)> {
  fn render_attributes_into<W: Write>(self, writer: &mut W) -> Result {
    // self.sort_by(|a, b| a.0.partial_cmp(&b.0));
    for (k, v) in self.iter() {
      match v {
        AttributeValue::None => (),
        _ => {
          write!(writer, " {}=\"", k)?;
          escape_html(&v.to_attribute_value(), writer)?;
          write!(writer, "\"")?;
        }
      }
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
