use std::{collections::HashMap, fmt::Write};

#[derive(Debug)]
pub enum Error {
  Array(Vec<String>),
  KeyValue(HashMap<String, String>),
  String(String),
}

impl Error {
  pub fn to_json(&self) -> String {
    match self {
      Self::Array(values) => {
        let mut buffer = "[".to_string();
        values.iter().enumerate().for_each(|(i, v)| {
          if i == 0 {
            write!(buffer, "\"{}\"", v).unwrap();
          } else {
            write!(buffer, ", \"{}\"", v).unwrap();
          }
        });
        write!(buffer, "]").unwrap();
        buffer
      }
      Self::KeyValue(key_values) => {
        let mut buffer = "{".to_string();
        key_values.iter().enumerate().for_each(|(i, (k, v))| {
          if i == 0 {
            write!(buffer, "\"{}\": \"{}\"", k, v).unwrap();
          } else {
            write!(buffer, ", \"{}\": \"{}\"", k, v).unwrap();
          }
        });
        write!(buffer, "}}").unwrap();
        buffer
      }
      Self::String(value) => format!("[\"{}\"]", value),
    }
  }
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::Array(values) => write!(f, "{:?}", values),
      Self::KeyValue(key_values) => write!(f, "{:?}", key_values),
      Self::String(value) => write!(f, "{}", value),
    }
  }
}

pub trait Validate: Sized {
  fn validate(values: serde_json::Value) -> Result<(), Error>;
}
