use serde::{Deserialize, Serialize};

use crate::migration::data_type::{DataType, DataTypeEnum};

mod data_type;

pub trait ToSql {
  fn to_sql(&self) -> String;
}

pub trait ToSqlMigration {
  fn sql_up(&self) -> Option<String> {
    None
  }

  fn sql_down(&self) -> Option<String> {
    None
  }
}

enum MigrationType {
  CreateTable,
  ChangeTable,
}

#[derive(Serialize, Deserialize)]
pub enum Collation {
  Utf8,
}

#[derive(Serialize, Deserialize)]
pub struct ColumnOptions {
  pub nullable: bool,
  pub visible: bool,
  pub comment: Option<String>,
  pub collation: Collation,
  pub primary_key: bool,
  pub unique: bool,
}

impl Default for ColumnOptions {
  fn default() -> Self {
    Self {
      nullable: true,
      visible: true,
      comment: None,
      collation: Collation::Utf8,
      primary_key: false,
      unique: false,
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct Column {
  pub name: String,
  pub data_type: DataType,
  pub options: ColumnOptions,
}

pub struct MigrationChange {
  columns: Vec<Column>,
  migration_type: MigrationType,
  table_name: String,
}

impl MigrationChange {
  pub fn create_table(table_name: &str) -> Self {
    Self {
      columns: vec![],
      migration_type: MigrationType::CreateTable,
      table_name: table_name.to_owned(),
    }
  }

  pub fn change_table(table_name: &str) -> Self {
    Self {
      columns: vec![],
      migration_type: MigrationType::ChangeTable,
      table_name: table_name.to_owned(),
    }
  }

  // TODO: Figure out this error `the trait bound `DataType: From<Option<T>>` is not satisfied` ... `the trait `From<Option<T>>` is not implemented for `DataType``
  // pub fn column<T>(&mut self, name: &str, default: Option<T>, options: ColumnOptions)
  // where
  //   T: Into<DataType>,
  // {
  //   self.columns.push(Column {
  //     data_type: default.into(),
  //     name: name.to_owned(),
  //     options,
  //   });
  // }

  pub fn varchar(
    &mut self,
    name: &str,
    default: Option<String>,
    length: u8,
    options: ColumnOptions,
  ) {
    self.columns.push(Column {
      data_type: DataType::Varchar(length, default),
      name: name.to_owned(),
      options,
    });
  }

  pub fn char(&mut self, name: &str, length: u8, default: Option<String>, options: ColumnOptions) {
    self.columns.push(Column {
      data_type: DataType::Char(length, default),
      name: name.to_owned(),
      options,
    });
  }

  pub fn uuid(&mut self, name: &str, generate_random: bool, options: ColumnOptions) {
    self.columns.push(Column {
      data_type: DataType::Uuid(generate_random),
      name: name.to_owned(),
      options,
    });
  }

  pub fn text(&mut self, name: &str, options: ColumnOptions) {
    self.columns.push(Column {
      data_type: DataType::Text,
      name: name.to_owned(),
      options,
    });
  }

  pub fn enum_<T>(&mut self, name: &str, default: Option<T>, options: ColumnOptions)
  where
    T: DataTypeEnum,
  {
    let default_str = if let Some(default) = default {
      Some(default.to_string())
    } else {
      None
    };

    self.columns.push(Column {
      data_type: DataType::Enum(T::enum_name().to_owned(), default_str),
      name: name.to_owned(),
      options,
    });
  }

  pub fn timestamp(
    &mut self,
    name: &str,
    default_current_timestamp: bool,
    on_update_current_timestamp: bool,
    options: ColumnOptions,
  ) {
    self.columns.push(Column {
      data_type: DataType::Timestamp(default_current_timestamp, on_update_current_timestamp),
      name: name.to_owned(),
      options,
    });
  }

  pub fn date(&mut self, name: &str, options: ColumnOptions) {
    self.columns.push(Column {
      data_type: DataType::Date,
      name: name.to_owned(),
      options,
    });
  }

  pub fn datetime(&mut self, name: &str, options: ColumnOptions) {
    self.columns.push(Column {
      data_type: DataType::DateTime,
      name: name.to_owned(),
      options,
    });
  }

  pub fn time(&mut self, name: &str, options: ColumnOptions) {
    self.columns.push(Column {
      data_type: DataType::Time,
      name: name.to_owned(),
      options,
    });
  }

  pub fn timestamps(&mut self) {
    self.timestamp("created_at", true, false, Default::default());
    self.timestamp("updated_at", true, true, Default::default());
  }
}

pub trait Migration {
  fn change() -> MigrationChange;
}

impl ToSqlMigration for MigrationChange {
  fn sql_up(&self) -> Option<String> {
    let _table_name = &self.table_name;
    match self.migration_type {
      MigrationType::CreateTable => todo!(),
      MigrationType::ChangeTable => todo!(),
    }
  }

  fn sql_down(&self) -> Option<String> {
    todo!()
  }
}

// #[cfg(test)]
// mod test {
//   use crate::migration::MigrationChange;

//   // #[test]
//   // fn test() {
//   //   let mut migration = MigrationChange::create_table("test");
//   //   migration.column::<u8>("u8", None, Default::default());
//   // }
// }
