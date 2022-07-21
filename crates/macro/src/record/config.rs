use std::{collections::HashMap, fs::read, path::Path};

use serde::Deserialize;

type ForeignKey = HashMap<String, Field>;

#[derive(Clone, Deserialize)]
pub struct Field {
  pub name: String,
  pub ty: String,
}

#[derive(Clone, Deserialize)]
pub struct Relation {
  pub table_name: String,
  pub foreing_key: ForeignKey,
}

#[derive(Clone, Deserialize)]
pub struct TableConfig {
  #[serde(default)]
  pub primary_key: Vec<Field>,
  #[serde(default)]
  pub columns: Vec<Field>,
  #[serde(default)]
  pub relations: Vec<Relation>,
  #[serde(default)]
  pub constraints: Vec<Field>,
}

#[derive(Default, Deserialize)]
pub struct RecordConfig {
  tables: HashMap<String, TableConfig>,
}

impl RecordConfig {
  pub fn table(&self, name: &str) -> Option<TableConfig> {
    self.tables.get(name).map(|v| v.clone())
  }
}

pub fn get_config() -> RecordConfig {
  let cwd = {
    let cwd = std::env::current_dir().expect("Could not read `cwd` env var");
    format!(
      "{}/records.json",
      cwd.to_str().expect("Could not convert `cwd` to str")
    )
  };
  let path = Path::new(cwd.as_str());

  if path.exists() {
    let content = read(path).expect("Could not read records.json");
    let tables: HashMap<String, TableConfig> =
      serde_json::from_slice(&content).expect("Could not parse records.json");

    RecordConfig { tables }
  } else {
    println!("records.json not found");
    RecordConfig::default()
  }
}
