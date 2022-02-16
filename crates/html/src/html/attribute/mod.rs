use crate::ToHtmlString;

#[cfg(feature = "time")]
use ::time_::{OffsetDateTime, PrimitiveDateTime};

#[cfg(feature = "time")]
mod time;

pub enum AttributeValue {
  Bool(bool),
  I8(i8),
  I16(i16),
  I32(i32),
  I64(i64),
  I128(i128),
  None,
  #[cfg(feature = "time")]
  OffsetDateTime(OffsetDateTime),
  #[cfg(feature = "time")]
  PrimitiveDateTime(PrimitiveDateTime),
  U8(u8),
  U16(u16),
  U32(u32),
  U64(u64),
  U128(u128),
  Text(String),
}

impl ToString for AttributeValue {
  fn to_string(&self) -> String {
    match self {
      AttributeValue::Bool(value) => value.to_string(),
      AttributeValue::I8(value) => value.to_string(),
      AttributeValue::I16(value) => value.to_string(),
      AttributeValue::I32(value) => value.to_string(),
      AttributeValue::I64(value) => value.to_string(),
      AttributeValue::I128(value) => value.to_string(),
      AttributeValue::None => String::new(),
      #[cfg(feature = "time")]
      AttributeValue::OffsetDateTime(value) => value.to_string(),
      #[cfg(feature = "time")]
      AttributeValue::PrimitiveDateTime(value) => value.to_string(),
      AttributeValue::U8(value) => value.to_string(),
      AttributeValue::U16(value) => value.to_string(),
      AttributeValue::U32(value) => value.to_string(),
      AttributeValue::U64(value) => value.to_string(),
      AttributeValue::U128(value) => value.to_string(),
      AttributeValue::Text(value) => value.to_string(),
    }
  }
}

impl From<bool> for AttributeValue {
  fn from(value: bool) -> Self {
    Self::Bool(value)
  }
}

impl From<Option<bool>> for AttributeValue {
  fn from(value: Option<bool>) -> Self {
    match value {
      Some(value) => value.into(),
      None => Self::None,
    }
  }
}

impl From<i8> for AttributeValue {
  fn from(value: i8) -> Self {
    Self::I8(value)
  }
}

impl From<Option<i8>> for AttributeValue {
  fn from(value: Option<i8>) -> Self {
    match value {
      Some(value) => value.into(),
      None => Self::None,
    }
  }
}

impl From<&str> for AttributeValue {
  fn from(value: &str) -> Self {
    Self::Text(value.to_owned())
  }
}

impl From<Option<&str>> for AttributeValue {
  fn from(value: Option<&str>) -> Self {
    match value {
      Some(value) => value.into(),
      None => Self::None,
    }
  }
}

impl From<String> for AttributeValue {
  fn from(value: String) -> Self {
    Self::Text(value)
  }
}

impl From<Option<String>> for AttributeValue {
  fn from(value: Option<String>) -> Self {
    match value {
      Some(value) => value.into(),
      None => Self::None,
    }
  }
}

impl ToHtmlString for Vec<(String, AttributeValue)> {
  fn render_into<W: std::fmt::Write>(self, buffer: &mut W) -> std::fmt::Result {
    for (key, value) in self {
      match value {
        AttributeValue::None | AttributeValue::Bool(true) => write!(buffer, "{}", key)?,
        _ => write!(buffer, r#"{}="{}""#, key, value.to_string())?,
      }
    }
    Ok(())
  }
}
