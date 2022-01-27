use serde::{Deserialize, Serialize};

use crate::migration::ToSqlMigration;

pub trait DataTypeEnum: ToString + ToSqlMigration {
  fn enum_name() -> &'static str;
  fn enum_values() -> Vec<&'static str>;
  fn from(value: &str) -> anyhow::Result<Self>
  where
    Self: Sized;
  fn into(&self) -> String;
}

#[derive(Serialize, Deserialize)]
pub enum DataType {
  // Integer
  I8(Option<i8>),
  I16(Option<i16>),
  I32(Option<i32>),
  I64(Option<i64>),
  I128(Option<i128>),
  Isize(Option<isize>),
  U8(Option<u8>),
  U16(Option<u16>),
  U32(Option<u32>),
  U64(Option<u64>),
  U128(Option<u128>),
  Usize(Option<usize>),

  // Float
  F32(Option<f32>),
  F64(Option<f64>),

  // Date and Time
  Date,
  DateTime,
  Time,
  Timestamp(bool, bool),

  // String
  Binary,
  Blob,
  Char(u8, Option<String>),
  Enum(String, Option<String>),
  Set,
  Varchar(u8, Option<String>),
  Text,
  Uuid(bool),

  // Others
  Boolean(Option<bool>),
}

impl From<Option<u8>> for DataType {
  fn from(item: Option<u8>) -> Self {
    Self::U8(item)
  }
}

impl From<Option<u16>> for DataType {
  fn from(item: Option<u16>) -> Self {
    Self::U16(item)
  }
}

impl From<Option<u32>> for DataType {
  fn from(item: Option<u32>) -> Self {
    Self::U32(item)
  }
}

impl From<Option<u64>> for DataType {
  fn from(item: Option<u64>) -> Self {
    Self::U64(item)
  }
}

impl From<Option<u128>> for DataType {
  fn from(item: Option<u128>) -> Self {
    Self::U128(item)
  }
}

impl From<Option<usize>> for DataType {
  fn from(item: Option<usize>) -> Self {
    Self::Usize(item)
  }
}

impl From<Option<i8>> for DataType {
  fn from(item: Option<i8>) -> Self {
    Self::I8(item)
  }
}

impl From<Option<i16>> for DataType {
  fn from(item: Option<i16>) -> Self {
    Self::I16(item)
  }
}

impl From<Option<i32>> for DataType {
  fn from(item: Option<i32>) -> Self {
    Self::I32(item)
  }
}

impl From<Option<i64>> for DataType {
  fn from(item: Option<i64>) -> Self {
    Self::I64(item)
  }
}

impl From<Option<i128>> for DataType {
  fn from(item: Option<i128>) -> Self {
    Self::I128(item)
  }
}

impl From<Option<isize>> for DataType {
  fn from(item: Option<isize>) -> Self {
    Self::Isize(item)
  }
}

impl From<Option<f32>> for DataType {
  fn from(item: Option<f32>) -> Self {
    Self::F32(item)
  }
}

impl From<Option<f64>> for DataType {
  fn from(item: Option<f64>) -> Self {
    Self::F64(item)
  }
}

impl From<Option<bool>> for DataType {
  fn from(item: Option<bool>) -> Self {
    Self::Boolean(item)
  }
}

// TODO: Hide behind feature
mod postgres {
  use super::DataType;
  use crate::migration::ToSql;
  use std::fmt::Write;

  // impl ToSqlMigration for dyn DataTypeEnum {
  //   fn sql_up() -> Option<String> {
  //     Some(format!(
  //       "CREATE TYPE {} ENUM ({})",
  //       Self::enum_name(),
  //       Self::enum_values()
  //         .iter()
  //         .map(|v| format!("'{}'", v))
  //         .collect::<Vec<_>>()
  //         .join(",")
  //     ))
  //   }
  //
  //   fn sql_down() -> Option<String> {
  //     Some(format!("DROP TYPE {}", Self::enum_name()))
  //   }
  // }

  impl ToSql for DataType {
    fn to_sql(&self) -> String {
      match self {
        DataType::I8(_) => todo!(),
        DataType::I16(value) => {
          if let Some(value) = value {
            format!("SMALLINT DEFAULT {}", value)
          } else {
            "SMALLINT".to_owned()
          }
        }
        DataType::I32(value) => {
          if let Some(value) = value {
            format!("INTEGER DEFAULT {}", value)
          } else {
            "INTEGER".to_owned()
          }
        }
        DataType::I64(value) => {
          if let Some(value) = value {
            format!("BIGINT DEFAULT {}", value)
          } else {
            "BIGINT".to_owned()
          }
        }
        DataType::I128(_) => todo!(),
        DataType::Isize(_) => todo!(),
        DataType::U8(_) => todo!(),
        DataType::U16(value) => {
          if let Some(value) = value {
            format!("SMALLSERIAL DEFAULT {}", value)
          } else {
            "SMALLSERIAL".to_owned()
          }
        }
        DataType::U32(value) => {
          if let Some(value) = value {
            format!("SERIAL DEFAULT {}", value)
          } else {
            "SERIAL".to_owned()
          }
        }
        DataType::U64(value) => {
          if let Some(value) = value {
            format!("BIGSERIAL DEFAULT {}", value)
          } else {
            "BIGSERIAL".to_owned()
          }
        }
        DataType::U128(_) => todo!(),
        DataType::Usize(_) => todo!(),
        DataType::F32(value) => {
          if let Some(value) = value {
            format!("REAL DEFAULT {}", value)
          } else {
            "REAL".to_owned()
          }
        }
        DataType::F64(value) => {
          if let Some(value) = value {
            format!("DOUBLE PRECISION DEFAULT {}", value)
          } else {
            "DOUBLE PRECISION".to_owned()
          }
        }
        DataType::Date => "DATE".to_owned(),
        DataType::DateTime => "TIMESTAMP".to_owned(),
        DataType::Time => "TIME".to_owned(),
        DataType::Timestamp(default_current_timestamp, on_update_current_timestamp) => {
          let mut buffer = "TIMESTAMP WITH TIME ZONE".to_owned();

          if *default_current_timestamp {
            write!(&mut buffer, " DEFAULT CURRENT_TIMESTAMP").unwrap();
          }

          if *on_update_current_timestamp {
            panic!("Update CURRENT_TIMESTAMP when a row is updated isn't supported in PostgreSQL at the moment");
          }

          buffer
        }
        DataType::Binary => todo!(),
        DataType::Blob => todo!(),
        DataType::Char(length, value) => {
          if let Some(value) = value {
            format!("CHAR({}) DEFAULT '{}'", length, value)
          } else {
            format!("CHAR({})", length)
          }
        }
        DataType::Enum(enum_name, value) => {
          if let Some(value) = value {
            format!("\"{}\" DEFAULT E'{}'", enum_name, value)
          } else {
            enum_name.to_owned()
          }
        }
        DataType::Set => todo!(),
        DataType::Varchar(length, value) => {
          if let Some(value) = value {
            format!("VARCHAR({}) DEFAULT {}", length, value)
          } else {
            format!("VARCHAR({})", length)
          }
        }
        DataType::Text => "TEXT".to_owned(),
        DataType::Uuid(generate_random) => {
          if *generate_random {
            "UUID gen_random_uuid()".to_owned()
          } else {
            "UUID".to_owned()
          }
        }
        DataType::Boolean(value) => {
          if let Some(value) = value {
            format!("BOOLEAN DEFAULT {}", value)
          } else {
            "BOOLEAN".to_owned()
          }
        }
      }
    }
  }
}
