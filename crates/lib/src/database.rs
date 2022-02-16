use std::{fmt::Display, marker::PhantomData};

#[derive(Clone, Copy)]
pub struct TableColumn {
  pub name: &'static str,
  pub is_primary_key: bool,
  pub sqlx_cast_as_underscore: bool,
}

impl Display for TableColumn {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.name)
  }
}

pub trait Queryable {
  fn columns() -> Vec<TableColumn>;
}

pub trait Table: Queryable {
  fn name() -> &'static str;
}

pub struct QueryBuilder<T, R>
where
  T: Table,
  R: Queryable,
{
  _marker_table: PhantomData<T>,
  _marker_queriable: PhantomData<R>,
}

impl<T, R> QueryBuilder<T, R>
where
  T: Table,
  R: Queryable,
{
  pub fn new() -> Self {
    Self {
      _marker_queriable: PhantomData,
      _marker_table: PhantomData,
    }
  }

  pub fn filter(self) -> Self {
    self
  }
}
