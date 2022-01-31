use sqlx::postgres::PgArguments;

pub mod migration;
// TODO: This module will be used to check the struct integrity against an offline representation of the database
mod backend;
mod metadata;
mod schema;
mod to_sql;

pub use to_sql::*;

pub trait Table {
  type PrimaryKeyType;

  /// The table name
  fn table_name() -> &'static str;
}

pub trait Record<'r>: Send + Unpin {
  type PrimaryKeyType: 'r
    + sqlx::Type<sqlx::Postgres>
    + std::marker::Sync
    + sqlx::Encode<'r, sqlx::Postgres>
    + Clone
    + std::marker::Send;
  type Values: ToArguments;
  // type Values: ToSql + ToArray<String>;

  /// Primary key name
  fn record_primary_key() -> &'static str;
  /// Primary key value
  fn record_primary_key_value(&self) -> Self::PrimaryKeyType;
  /// Column names
  fn record_columns() -> Vec<&'static str>;
  /// Column values
  fn record_values(&self) -> Self::Values;
  /// Column values as args
  fn record_values_as_args(&self) -> PgArguments;
}
