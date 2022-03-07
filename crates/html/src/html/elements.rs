use crate::{Attributes};

use super::children::Children;

pub struct Element {
  pub attributes: Attributes,
  pub children: Children,
  pub name: &'static str,
}
