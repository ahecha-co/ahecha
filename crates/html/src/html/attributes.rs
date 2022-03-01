#[derive(Clone)]
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

#[derive(Default, Clone)]
pub struct Attributes {
  pub attrs: Vec<(String, AttributeValue)>,
}

impl Attributes {
  pub fn set<K, V>(mut self, key: K, value: V) -> Self
  where
    K: Into<String>,
    V: Into<AttributeValue>,
  {
    self.attrs.push((key.into(), value.into()));
    self
  }

  pub fn is_empty(&self) -> bool {
    self.attrs.is_empty()
  }
}

impl<V> From<V> for AttributeValue
where
  V: std::fmt::Display,
{
  fn from(item: V) -> Self {
    let value = item.to_string();

    if value.is_empty() {
      AttributeValue::None
    } else if let Ok(boolean) = value.parse::<bool>() {
      AttributeValue::Bool(boolean)
    } else {
      AttributeValue::String(value)
    }
  }
}
