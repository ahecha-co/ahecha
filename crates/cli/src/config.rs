use std::{
  collections::HashMap,
  fs::{self, read},
  path::Path,
};

use serde::{Deserialize, Serialize};
use serde_json::Map;
use sqlx::PgPool;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Column {
  pub name: String,
  pub ty: String,
  pub is_nullable: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TableConfig {
  #[serde(default)]
  pub columns: Vec<Column>,
  #[serde(default)]
  pub primary_keys: Vec<String>,
  #[serde(default)]
  pub constraints: Vec<String>,
}

#[derive(Default)]
pub struct RecordConfig {
  tables: HashMap<String, TableConfig>,
}

impl RecordConfig {
  pub fn table(&self, name: &str) -> Option<TableConfig> {
    self.tables.get(name).map(|v| v.clone())
  }

  fn set_table(&mut self, name: &str, table: TableConfig) {
    self.tables.insert(name.to_owned(), table);
  }

  fn to_json(&self) -> String {
    let mut json = Map::new();
    for (name, config) in self.tables.iter() {
      json.insert(
        name.clone(),
        serde_json::from_str(&serde_json::to_string(&config).unwrap()).unwrap(),
      );
    }

    serde_json::to_string_pretty(&json).unwrap()
  }
}

pub(crate) fn config_path() -> String {
  let cwd = std::env::current_dir().expect("Could not read `cwd` env var");
  format!(
    "{}/records.json",
    cwd.to_str().expect("Could not convert `cwd` to str")
  )
}

pub fn get_config() -> RecordConfig {
  let file = config_path();
  let path = Path::new(file.as_str());

  if path.exists() {
    let content = read(path).expect("Could not read records.json");
    let tables: HashMap<String, TableConfig> =
      serde_json::from_slice(&content).expect("Could not parse records.json");

    RecordConfig { tables }
  } else {
    RecordConfig::default()
  }
}

#[derive(sqlx::FromRow)]
struct TableInfo {
  table_name: Option<String>,
  column_name: Option<String>,
  udt_name: Option<String>,
  is_nullable: Option<String>,
}

pub async fn sync_config_from_db() -> anyhow::Result<()> {
  let pool = PgPool::connect(&std::env::var("DATABASE_URL")?).await?;
  let config = get_config();

  let rows = sqlx::query_as(
    r#"
      SELECT tables.table_name, columns.column_name, columns.udt_name, columns.is_nullable
      FROM information_schema.tables AS tables
      JOIN information_schema.columns as columns ON tables.table_name = columns.table_name
      WHERE tables.table_schema NOT IN ('pg_catalog', 'information_schema')
      ORDER BY columns.ordinal_position
    "#,
  )
  .fetch_all(&pool)
  .await?;

  write_config(merge_config(config, rows))
}

fn merge_config(mut config: RecordConfig, rows: Vec<TableInfo>) -> RecordConfig {
  for row in rows.iter() {
    let table_name = row
      .table_name
      .as_ref()
      .expect("Result from `information_schema.tables` `table_name` cannot be null");

    if let Some(mut table) = config.table(&table_name) {
      let name = row
        .column_name
        .as_ref()
        .expect("Result from `information_schema.columns` `column_name` cannot be null")
        .to_owned();
      let ty = row
        .udt_name
        .as_ref()
        .expect("Result from `information_schema.columns` `column_name` cannot be null")
        .to_owned();
      let is_nullable = row
        .is_nullable
        .as_ref()
        .expect("Result from `information_schema.columns` `is_nullable` cannot be null")
        .to_owned();

      let column = Column {
        name: name.clone(),
        ty,
        is_nullable: is_nullable == "YES",
      };

      if let Some(position) = table.columns.iter().position(|f| f.name == name) {
        table.columns.insert(position, column);
      } else {
        table.columns.push(column);
      }

      config.set_table(&table_name, table);
    }
  }

  for (table_name, table_config) in config.tables.iter_mut() {
    let columns = rows
      .iter()
      .filter(|f| f.table_name == Some(table_name.to_owned()))
      .map(|f| f.column_name.as_ref().unwrap().to_owned())
      .collect::<Vec<_>>();

    table_config.columns = table_config
      .columns
      .iter()
      .filter(|f| columns.contains(&f.name))
      .map(|f| f.clone())
      .collect::<Vec<_>>();

    table_config.columns.sort_by(|a, b| {
      let res_a = columns
        .iter()
        .position(|c| c == &a.name)
        .unwrap_or_else(|| usize::MAX);
      let res_b = columns
        .iter()
        .position(|c| c == &b.name)
        .unwrap_or_else(|| usize::MAX);

      res_a.partial_cmp(&res_b).unwrap()
    });
  }

  config
}

fn write_config(config: RecordConfig) -> anyhow::Result<()> {
  let content = config.to_json();
  fs::write(config_path(), content)?;

  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_load_config() {
    let config = get_config();
    assert!(config.table("users").is_some());
  }

  #[test]
  fn test_load_primary_key() {
    let config = get_config().table("users").unwrap();
    assert_eq!(config.primary_keys, vec!["id".to_owned()]);
  }

  #[test]
  fn test_load_constraints() {
    let config = get_config().table("users").unwrap();
    assert_eq!(config.constraints, vec!["tenant_id".to_owned()]);
  }

  #[test]
  fn test_load_columns() {
    let config = get_config().table("users").unwrap();
    assert_eq!(
      config.columns,
      vec![
        Column {
          name: "id".to_owned(),
          ty: "uuid".to_owned(),
          is_nullable: false,
        },
        Column {
          name: "name".to_owned(),
          ty: "varchar".to_owned(),
          is_nullable: false,
        },
        Column {
          name: "age".to_owned(),
          ty: "int4".to_owned(),
          is_nullable: true,
        },
        Column {
          name: "tenant_id".to_owned(),
          ty: "varchar".to_owned(),
          is_nullable: false,
        },
      ]
    );
  }

  #[test]
  fn test_merge_config() {
    let rows = vec![TableInfo {
      table_name: Some("users".to_owned()),
      column_name: Some("id".to_owned()),
      udt_name: Some("uuid".to_owned()),
      is_nullable: Some("NO".to_owned()),
    }];
    let config = merge_config(
      {
        let mut config = RecordConfig::default();
        config.set_table(
          "users",
          TableConfig {
            columns: vec![],
            primary_keys: vec![],
            constraints: vec![],
          },
        );
        config
      },
      rows,
    );
    assert!(config.table("users").is_some());
    assert_eq!(
      config.table("users").unwrap().columns,
      vec![Column {
        name: "id".to_owned(),
        ty: "uuid".to_owned(),
        is_nullable: false,
      }]
    )
  }

  #[test]
  fn test_merge_only_existing_tables_config() {
    let rows = vec![TableInfo {
      table_name: Some("users".to_owned()),
      column_name: Some("id".to_owned()),
      udt_name: Some("uuid".to_owned()),
      is_nullable: Some("NO".to_owned()),
    }];
    let config = merge_config(RecordConfig::default(), rows);
    assert!(config.table("users").is_none());
  }
}
