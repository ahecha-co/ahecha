use crate::{Attributes, RenderString};

use super::children::Children;
use std::fmt::Write;

pub struct Element {
  pub attributes: Attributes,
  pub children: Children,
  pub name: &'static str,
}

impl ToString for Element {
  fn to_string(&self) -> String {
    let mut buffer = String::new();

    write!(&mut buffer, "<{}", self.name).unwrap();

    if !self.attributes.is_empty() {
      self.attributes.clone().render_into(&mut buffer).unwrap();
    }

    if self.children.is_empty() {
      write!(&mut buffer, "/>").unwrap();
    } else {
      let children = self
        .children
        .children
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<_>>()
        .join("\n");
      write!(&mut buffer, ">{}</{}>", children, self.name).unwrap();
    }

    buffer
  }
}
