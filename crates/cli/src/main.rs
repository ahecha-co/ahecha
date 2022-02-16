use std::{
  fs::{create_dir_all, File},
  io::Write,
  path::Path,
};

use clap::{app_from_crate, arg, App};

enum DataType {
  Bool,
  Char,
  SmallInt,
  SmallSerial,
  Int,
  Serial,
  OID,
  BigInt,
  BigSerial,
  Real,
  DoublePrecision,
  Varchar,
  Text,
  Bytea,
  Timestamp,
  TimestampWithTimeZone,
}

impl TryFrom<String> for DataType {
  type Error = String;

  fn try_from(value: String) -> Result<Self, Self::Error> {
    match value.to_lowercase().as_str() {
      "bool" => Ok(DataType::Bool),
      "bpchar" => Ok(DataType::Char),
      "smallserial" => Ok(DataType::SmallSerial),
      "serial4" => Ok(DataType::Serial),
      _ => Err(format!("Type `{}` is not implemented.", value)),
    }
  }
}

impl ToString for DataType {
  fn to_string(&self) -> String {
    match self {
      DataType::Bool => "bool",
      DataType::Char => "u8",
      DataType::SmallInt => "i16",
      DataType::SmallSerial => "u16",
      DataType::Int => "i32",
      DataType::Serial => "u32",
      DataType::OID => "i32",
      DataType::BigInt => "i64",
      DataType::BigSerial => "u64",
      DataType::Real => "f32",
      DataType::DoublePrecision => "f64",
      DataType::Varchar => "String",
      DataType::Text => "String",
      DataType::Bytea => "Vec<u8>",
      DataType::Timestamp => "DateTime<Utc>",
      DataType::TimestampWithTimeZone => "DateTime<Utc>",
    }
    .to_owned()
  }
}

struct Column {
  column_name: String,
  data_type: DataType,
  is_nullable: bool,
  column_default: String,
}

impl ToString for Column {
  fn to_string(&self) -> String {
    format!("\t{}: {},", &self.column_name, &self.data_type.to_string())
  }
}

struct Table {
  name: String,
  model_name: String,
  fields: Vec<Column>,
}

impl ToString for Table {
  fn to_string(&self) -> String {
    format!(
      "// Table: {}\npub struct {} {{\n{}\n}}",
      &self.table,
      &self.name,
      &self
        .fields
        .iter()
        .map(|f| f.to_string())
        .collect::<Vec<_>>()
        .join("\n")
    )
  }
}

fn main() {
  let matches = app_from_crate!().subcommand(
    App::new("generate")
      .about("Generate new code (short-cut alias: \"g\"")
        .subcommand(
          App::new("model")
          .about("Generates a new model")
          .arg(arg!(<NAME> "Pass the model name, either CamelCased or under_scored, and an optional list of attribute pairs as arguments."))
          .arg(arg!(-t --table <TABLE> "Name of the table in the database")),
        )
  ).get_matches();

  if let Some(matches) = matches.subcommand_matches("generate") {
    if let Some(matches) = matches.subcommand_matches("model") {
      let name = if let Some(name) = matches.value_of("NAME") {
        name
      } else {
        panic!("No name found");
      };

      let table = if let Some(table) = matches.value_of("table") {
        table
      } else {
        panic!("No table found");
      };

      let model = Table {
        name: name.to_owned(),
        table: table.to_owned(),
        fields: vec![
          Column {
            column_name: "field1".to_owned(),
            data_type: DataType::BigInt,
          },
          Column {
            column_name: "field2".to_owned(),
            data_type: DataType::Bool,
          },
        ],
      };

      let model_path = "./src/models";
      let model_file = format!("{}/{}.rs", &model_path, &name);
      create_dir_all(model_path).expect("Directory `src/models` to be created");

      if Path::new(&model_file).exists() {
        panic!("Model file already exists.");
      }

      let mut file = File::create(&model_file).expect("Unable to create model file");
      file
        .write_all(model.to_string().as_bytes())
        .expect("Unable to write model data");

      // TODO: Find the .env and Cargo.toml recursively to get the project root
      // TODO: Load the .env file according to the AHECHA_ENV=[production|development]
      // TODO: Get the schema for the table
      // TODO: Write the schema of the table to the db/schema/{table}.json
      // TODO: Create the file src/models/{table.underscored()}.rs
      // TODO: Generate the struct based from the schema
    } else {
      panic!("No model found");
    }
  } else {
    panic!("No generate found");
  }
}
