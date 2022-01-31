#[cfg(feature = "chrono")]
use sqlx::types::chrono::{DateTime, Utc};
use std::fmt::{Result, Write};

use crate::{escape_html, html::AttributeValue, SerializableAttributeValue};

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

macro_rules! impl_serializable_attribute_value {
  ($($t:ty),*) => {
    $(impl From<$t> for SerializableAttributeValue {
      fn from(item: $t) -> SerializableAttributeValue {
        SerializableAttributeValue(Some(item.to_string()))
      }
    })*

    $(impl From<Option<$t>> for SerializableAttributeValue {
      fn from(item: Option<$t>) -> SerializableAttributeValue {
        match item {
          Some(item) => item.into(),
          None => SerializableAttributeValue(None),
        }
      }
    })*
  };
}

macro_rules! impl_serializable_attribute_value_with_into {
  ($($t:ty),*) => {
    $(impl_serializable_attribute_value!($t);)*

    $(impl From<SerializableAttributeValue> for $t {
      fn from(item: SerializableAttributeValue) -> $t {
        match item.0 {
          Some(s) => s.parse().unwrap(),
          None => unimplemented!(),
        }
      }
    })*

    $(impl From<SerializableAttributeValue> for Option<$t> {
      fn from(item: SerializableAttributeValue) -> Option<$t> {
        match item.0 {
          Some(s) => Some(s.parse().unwrap()),
          None => None,
        }
      }
    })*
  };
}

impl RenderAttributeValue for SerializableAttributeValue {
  fn to_attribute_value(&self) -> String {
    self.to_string()
  }
}

impl_serializable_attribute_value!(&&str, &str, &String, String, bool);
impl_serializable_attribute_value_with_into!(u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);

#[cfg(feature = "chrono")]
impl_serializable_attribute_value_with_into!(DateTime<Utc>);

#[cfg(feature = "time")]
mod time_impl {
  use super::*;
  use sqlx::types::time::{Date, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset};
  impl_serializable_attribute_value!(Date);
  impl_serializable_attribute_value!(OffsetDateTime);
  impl_serializable_attribute_value!(PrimitiveDateTime);
  impl_serializable_attribute_value!(Time);
  impl_serializable_attribute_value!(UtcOffset);

  impl From<SerializableAttributeValue> for OffsetDateTime {
    fn from(item: SerializableAttributeValue) -> OffsetDateTime {
      match item.0 {
        Some(s) => OffsetDateTime::parse(s, "%F%T%z").unwrap(),
        None => unimplemented!(),
      }
    }
  }

  impl From<SerializableAttributeValue> for Option<OffsetDateTime> {
    fn from(item: SerializableAttributeValue) -> Option<OffsetDateTime> {
      match item.0 {
        Some(s) => Some(OffsetDateTime::parse(s, "%F%T%z").unwrap()),
        None => None,
      }
    }
  }
}

// TODO: Find a way to implement this for serde
// impl<T> From<T> for SerializableAttributeValue
// where
//   T: Serialize,
// {
//   fn from(item: T) -> SerializableAttributeValue {
//     Some(item.to_string())
//   }
// }

// impl<T> From<Option<T>> for SerializableAttributeValue
// where
//   T: Serialize,
// {
//   fn from(item: Option<T>) -> SerializableAttributeValue {
//     match item {
//       Some(item) => item.to_string().into(),
//       None => SerializableAttributeValue(None),
//     }
//   }
// }

impl RenderAttributes for Vec<(String, SerializableAttributeValue)> {
  fn render_attributes_into<W: Write>(self, writer: &mut W) -> Result {
    // self.sort_by(|a, b| a.0.partial_cmp(&b.0));
    for (k, v) in self.iter() {
      match v.0 {
        None => (),
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
