use std::fmt::{Result, Write};

trait CreateTable: CreateTableInternal {
  fn create_table(&self) -> String {
    let mut buffer = String::new();
    self
      .create_table_internal(&mut buffer)
      .expect("Failed to write to table buffer");
    buffer
  }
}

trait CreateTableInternal {
  fn create_table_internal<W>(&self, w: &mut W) -> Result
  where
    W: Write;
}

struct Table {
  name: String,
  columns: Vec<Column>,
}

impl CreateTable for Table {}

impl CreateTableInternal for Table {
  fn create_table_internal<W>(&self, w: &mut W) -> Result
  where
    W: Write,
  {
    write!(w, "CREATE TABLE {} (", self.name)?;
    for (i, column) in self.columns.iter().enumerate() {
      if i > 0 {
        write!(w, ", ")?;
      }
      column.create_table_internal(w)?;
    }

    Ok(())
  }
}

struct Column {
  name: String,
  data_type: DataType,
  optional: bool,
}

impl CreateTableInternal for Column {
  fn create_table_internal<W>(&self, w: &mut W) -> Result
  where
    W: Write,
  {
    write!(
      w,
      "{} {} ",
      self.name,
      if self.optional { "NULL" } else { "NOT NULL" }
    )?;

    self.data_type.create_table_internal(w)?;
    Ok(())
  }
}

enum DataType {
  I8,
  I16,
  I32,
  I64,
  // I128,
  U8,
  U16,
  U32,
  U64,
  // U128,
  F32,
  F64,
  DECIMAL(u16, u16),
  String(u64),
  Enum(Vec<String>),
  Bool,
  // Date,
  // DateTime,
  // Time,
  // Timestamp,
  // Interval,
  // Json,
  // Jsonb,
  // Uuid,
  // Array(Box<DataType>),
  // Nullable(Box<DataType>),
  // Enum(Vec<String>),
  // Range(Box<DataType>),
  // Composite(Vec<Column>),
}

// TODO: This needs to be implemented for each database engine

#[cfg(feature = "mysql")]
mod mysql {
  use super::{CreateTableInternal, DataType, Result, Write};

  impl CreateTableInternal for DataType {
    fn create_table_internal<W>(&self, w: &mut W) -> Result
    where
      W: Write,
    {
      match self {
        DataType::I8 => write!(w, "TINYINT"),
        DataType::I16 => write!(w, "SMALLINT"),
        DataType::I32 => write!(w, "INT"),
        DataType::I64 => write!(w, "BIGINT"),
        DataType::U8 => write!(w, "TINYINT UNSIGNED"),
        DataType::U16 => write!(w, "SMALLINT UNSIGNED"),
        DataType::U32 => write!(w, "INT UNSIGNED"),
        DataType::U64 => write!(w, "BIGINT UNSIGNED"),
        DataType::F32 => write!(w, "FLOAT"),
        DataType::F64 => write!(w, "DOUBLE"),
        DataType::DECIMAL(ld, rd) => write!(w, "DECIMAL({ld}, {rd})"),
        DataType::String(len) => {
          let len = *len;

          if len <= 255 {
            write!(w, "VARCHAR({})", len)
          } else if len <= 65535 {
            write!(w, "TEXT")
          } else if len <= 4294967295 {
            write!(w, "MEDIUMTEXT")
          } else {
            write!(w, "LONGTEXT")
          }
        }
        DataType::Enum(values) => {
          write!(w, "ENUM(")?;
          for (i, value) in values.iter().enumerate() {
            if i > 0 {
              write!(w, ", ")?;
            }
            write!(w, "'{}'", value)?;
          }
          write!(w, ")")
        }
        DataType::Bool => write!(w, "BOOLEAN"),
      }
    }
  }
}

// #[cfg(feature = "postgres")]
mod postgres {
  use super::{CreateTableInternal, DataType, Result, Write};

  impl CreateTableInternal for DataType {
    fn create_table_internal<W>(&self, w: &mut W) -> Result
    where
      W: Write,
    {
      match self {
        DataType::I8 => write!(w, "SMALLINT"),
        DataType::I16 => write!(w, "SMALLINT"),
        DataType::I32 => write!(w, "INTEGER"),
        DataType::I64 => write!(w, "BIGINT"),
        DataType::U8 => write!(w, "SMALLINT"),
        DataType::U16 => write!(w, "SMALLINT"),
        DataType::U32 => write!(w, "INTEGER"),
        DataType::U64 => write!(w, "BIGINT"),
        DataType::F32 => write!(w, "REAL"),
        DataType::F64 => write!(w, "DOUBLE PRECISION"),
        DataType::DECIMAL(ld, rd) => write!(w, "DECIMAL({ld}, {rd})"),
        DataType::String(_) => todo!(),
        DataType::Enum(_) => todo!(),
        DataType::Bool => todo!(),
      }
    }
  }
}
