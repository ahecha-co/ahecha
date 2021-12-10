use ahecha_tuple_list::TupleList;

use super::RenderNode;

impl<Head, Tail> RenderNode for (Head, Tail)
where
  Head: RenderNode,
  Tail: RenderNode + TupleList,
{
  fn render_into(&self, parent: &web_sys::Node) {
    self.0.render_into(&parent);
    self.1.render_into(&parent);
  }

  fn render(&self) -> web_sys::Node {
    unimplemented!()
  }
}
