use super::children::Children;
use crate::Attributes;

#[derive(Debug, Clone)]
pub struct Element {
  pub attributes: Attributes,
  pub children: Children,
  pub name: &'static str,
}
