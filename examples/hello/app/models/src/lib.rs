use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Post<'a> {
  pub id: usize,
  pub title: &'a str,
  pub body: &'a str,
  pub image: &'a str,
}
