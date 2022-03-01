use crate::Node;

#[derive(Default)]
pub struct Children {
  pub children: Vec<Node>,
}

impl Children {
  pub fn set<C>(mut self, child: C) -> Self
  where
    C: Into<Node>,
  {
    self.children.push(child.into());
    self
  }

  pub fn set_node(mut self, child: Node) -> Self {
    self.children.push(child);
    self
  }

  pub fn is_empty(&self) -> bool {
    self.children.is_empty()
  }
}
