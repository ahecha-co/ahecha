#[derive(Debug, Clone, PartialEq)]
pub enum AttributeValue {
  Bool(bool),
  None,
  String(String),
}

impl ToString for AttributeValue {
  fn to_string(&self) -> String {
    match self {
      AttributeValue::Bool(value) => value.to_string(),
      AttributeValue::None => "".to_owned(),
      AttributeValue::String(text) => text.clone(),
    }
  }
}

#[derive(Default, Debug, Clone)]
pub struct Attributes {
  pub attrs: Vec<(String, AttributeValue)>,
}

impl Attributes {
  pub fn set<K, V>(mut self, tuple: Option<(K, V)>) -> Self
  where
    K: Into<String>,
    V: Into<AttributeValue>,
  {
    if let Some((key, value)) = tuple {
      self.attrs.push((key.into(), value.into()));
    }
    self
  }

  pub fn is_empty(&self) -> bool {
    self.attrs.is_empty()
  }
}

macro_rules! impl_into_attribute_value {
  ($($ty: ty),*) => {
    $(
      impl From<$ty> for AttributeValue
      {
        fn from(item: $ty) -> Self {
          let value = format!("{}", item);

          if value.is_empty() {
            AttributeValue::None
          } else if let Ok(boolean) = value.parse::<bool>() {
            AttributeValue::Bool(boolean)
          } else {
            AttributeValue::String(value)
          }
        }
      }

      impl From<Option<$ty>> for AttributeValue
      {
        fn from(item: Option<$ty>) -> Self {
          match item {
            Some(value) => value.into(),
            None => AttributeValue::None,
          }
        }
      }

      impl From<&$ty> for AttributeValue
      {
        fn from(item: &$ty) -> Self {
          let value = format!("{}", item);

          if value.is_empty() {
            AttributeValue::None
          } else if let Ok(boolean) = value.parse::<bool>() {
            AttributeValue::Bool(boolean)
          } else {
            AttributeValue::String(value)
          }
        }
      }

      impl From<Option<&$ty>> for AttributeValue
      {
        fn from(item: Option<&$ty>) -> Self {
          match item {
            Some(value) => value.into(),
            None => AttributeValue::None,
          }
        }
      }
    )*
  };
}

impl_into_attribute_value!(
  bool, i8, i16, i32, i64, i128, f32, f64, u8, u16, u32, u64, u128, &str, String
);

impl<T> From<Result<String, T>> for AttributeValue
where
  T: ToString,
{
  fn from(item: Result<String, T>) -> Self {
    match item {
      Ok(value) => value.into(),
      Err(e) => {
        println!("{}", e.to_string());
        AttributeValue::None
      }
    }
  }
}

#[cfg(feature = "time")]
mod _time {
  use time::OffsetDateTime;

  use super::*;

  impl_into_attribute_value!();

  impl From<OffsetDateTime> for AttributeValue {
    fn from(item: OffsetDateTime) -> Self {
      let value = format!("{}", item);

      if value.is_empty() {
        AttributeValue::None
      } else if let Ok(boolean) = value.parse::<bool>() {
        AttributeValue::Bool(boolean)
      } else {
        AttributeValue::String(value)
      }
    }
  }

  impl From<Option<OffsetDateTime>> for AttributeValue {
    fn from(item: Option<OffsetDateTime>) -> Self {
      match item {
        Some(value) => value.into(),
        None => AttributeValue::None,
      }
    }
  }

  impl From<&OffsetDateTime> for AttributeValue {
    fn from(item: &OffsetDateTime) -> Self {
      let value = format!("{}", item);

      if value.is_empty() {
        AttributeValue::None
      } else if let Ok(boolean) = value.parse::<bool>() {
        AttributeValue::Bool(boolean)
      } else {
        AttributeValue::String(value)
      }
    }
  }

  impl From<Option<&OffsetDateTime>> for AttributeValue {
    fn from(item: Option<&OffsetDateTime>) -> Self {
      match item {
        Some(value) => value.into(),
        None => AttributeValue::None,
      }
    }
  }
}
