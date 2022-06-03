use crate::Node;

#[derive(Default, Debug, Clone)]
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

  pub fn remove(&mut self, index: usize) {
    self.children.remove(index);
  }

  pub fn iter(&self) -> std::slice::Iter<Node> {
    self.children.iter()
  }

  pub fn iter_mut(&mut self) -> std::slice::IterMut<Node> {
    self.children.iter_mut()
  }
}
