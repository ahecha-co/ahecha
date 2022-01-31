#[derive(PartialEq)]
pub enum DataTypes {
  Bool,
  Bytes,
  I8,
  I16,
  I32,
  U32,
  I64,
  F32,
  F64,
  String,
  HashMap,
  SystemTime,
  IpAddr,
  // Feature flags
  Uuid,
  TimestampWithTimeZone,
}

impl TryFrom<String> for DataTypes {
  type Error = String;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    match value.as_str() {
      "bool" => Ok(Self::Bool),
      "Vec<u8>" => Ok(Self::Bytes),
      "&[u8]" => Ok(Self::Bytes),
      "i8" => Ok(Self::I8),
      "i16" => Ok(Self::I16),
      "i32" => Ok(Self::I32),
      "u32" => Ok(Self::U32),
      "i64" => Ok(Self::I64),
      "f32" => Ok(Self::F32),
      "f64" => Ok(Self::F64),
      "String" => Ok(Self::String),
      "&str" => Ok(Self::String),
      "HashMap<String, Option<String>>" => Ok(Self::HashMap),
      "SystemTime" => Ok(Self::SystemTime),
      "IpAddr" => Ok(Self::IpAddr),
      "Uuid" => Ok(Self::Uuid),
      "DateTime<Utc>" => Ok(Self::TimestampWithTimeZone),
      _ => Err(format!("Type {} isn't supported at the moment", value)),
    }
  }
}

pub struct StructFieldMetadata {
  ident: String,
  ty: DataTypes,
}

pub struct StructMetadata {
  ident: String,
  fields: Vec<StructFieldMetadata>,
}

pub struct TableColumnMetadata {
  name: String,
  ty: DataTypes,
  is_primary_key: bool,
  is_indexed: bool,
}

pub struct TableMetadata {
  name: String,
  columns: Vec<TableColumnMetadata>,
}

pub fn validate(table: TableMetadata, record: StructMetadata) -> Result<(), Vec<String>> {
  let mut errors = vec![];

  for field in record.fields {
    if let Some(column) = table.columns.iter().find(|c| c.name == field.ident) {
      if column.ty != field.ty {
        errors.push(format!(
          "The field `{}` from the struct `{}` type doesn't match the type from the `{}` column",
          &field.ident, &record.ident, &table.name
        ));
      }
    } else {
      errors.push(format!(
        "The field `{}` from the struct `{}` isn't a column of the table `{}`",
        &field.ident, &record.ident, &table.name
      ));
    }
  }

  if errors.is_empty() {
    Ok(())
  } else {
    Err(errors)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn check_that_column_field_names_matches() {
    let table = TableMetadata {
      name: "test".to_owned(),
      columns: vec![TableColumnMetadata {
        name: "field1".to_owned(),
        ty: DataTypes::Bool,
        is_primary_key: false,
        is_indexed: false,
      }],
    };
    let record = StructMetadata {
      ident: "Test".to_owned(),
      fields: vec![StructFieldMetadata {
        ident: "field1".to_owned(),
        ty: DataTypes::Bool,
      }],
    };

    assert!(validate(table, record).is_ok());
  }

  #[test]
  fn check_all_fields_must_be_present_in_the_table() {
    let table = TableMetadata {
      name: "test".to_owned(),
      columns: vec![TableColumnMetadata {
        name: "field1".to_owned(),
        ty: DataTypes::Bool,
        is_primary_key: false,
        is_indexed: false,
      }],
    };
    let record = StructMetadata {
      ident: "Test".to_owned(),
      fields: vec![
        StructFieldMetadata {
          ident: "field1".to_owned(),
          ty: DataTypes::Bool,
        },
        StructFieldMetadata {
          ident: "field2".to_owned(),
          ty: DataTypes::I16,
        },
      ],
    };

    assert!(validate(table, record).is_err());
  }

  #[test]
  fn check_field_column_types_matches() {
    let table = TableMetadata {
      name: "test".to_owned(),
      columns: vec![TableColumnMetadata {
        name: "field1".to_owned(),
        ty: DataTypes::Bool,
        is_primary_key: false,
        is_indexed: false,
      }],
    };
    let record = StructMetadata {
      ident: "Test".to_owned(),
      fields: vec![StructFieldMetadata {
        ident: "field1".to_owned(),
        ty: DataTypes::I16,
      }],
    };

    assert!(validate(table, record).is_err());
  }
}
