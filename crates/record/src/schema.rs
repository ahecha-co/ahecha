use serde::{Deserialize, Serialize};

use crate::migration::Column;

#[derive(Serialize, Deserialize)]
pub struct Table {
  name: String,
  columns: Vec<Column>,
}

#[derive(Serialize, Deserialize)]
pub struct Schema {
  tables: Vec<Table>,
}
