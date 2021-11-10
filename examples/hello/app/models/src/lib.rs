use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Post<'a> {
  pub id: usize,
  pub title: &'a str,
  pub body: &'a str,
  pub image: &'a str,
}

impl<'a> Default for Post<'a> {
  fn default() -> Self {
    Post {
      id: 0,
      title: "",
      body: "",
      image: "",
    }
  }
}
