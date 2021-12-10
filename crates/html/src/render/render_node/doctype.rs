use crate::HtmlDoctype;

use super::RenderNode;

impl<T> RenderNode for HtmlDoctype<T>
where
  T: RenderNode,
{
  fn render_into(&self, _parent: &web_sys::Node) {
    panic!("Document must be a root element")
  }

  fn render(&self) -> web_sys::Node {
    unimplemented!()
  }
}
