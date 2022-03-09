use crate::Node;

pub trait Component {
  fn view(&self) -> Node;
}

impl<C> From<C> for Node
where
  C: Component,
{
  fn from(item: C) -> Self {
    item.view()
  }
}
