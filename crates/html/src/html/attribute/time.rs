use ::time_::{OffsetDateTime, PrimitiveDateTime};

use super::AttributeValue;

impl From<OffsetDateTime> for AttributeValue {
  fn from(value: OffsetDateTime) -> Self {
    Self::OffsetDateTime(value)
  }
}

impl From<Option<OffsetDateTime>> for AttributeValue {
  fn from(value: Option<OffsetDateTime>) -> Self {
    match value {
      Some(value) => value.into(),
      None => Self::None,
    }
  }
}

impl From<PrimitiveDateTime> for AttributeValue {
  fn from(value: PrimitiveDateTime) -> Self {
    Self::PrimitiveDateTime(value)
  }
}

impl From<Option<PrimitiveDateTime>> for AttributeValue {
  fn from(value: Option<PrimitiveDateTime>) -> Self {
    match value {
      Some(value) => value.into(),
      None => Self::None,
    }
  }
}
