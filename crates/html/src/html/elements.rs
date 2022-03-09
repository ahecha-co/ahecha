use crate::Attributes;

use super::children::Children;

#[derive(Debug, Clone)]
pub struct Element {
  pub attributes: Attributes,
  pub children: Children,
  pub name: &'static str,
}
