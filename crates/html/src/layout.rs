use crate::Node;

pub trait Layout {
  fn render(head: Option<Node>, body: Node) -> Node;
}
