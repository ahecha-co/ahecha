use super::RenderNode;

impl RenderNode for String {
  fn render(&self) -> web_sys::Node {
    let text = gloo_utils::document().create_text_node(self.as_str());
    text.into()
  }
}

impl RenderNode for &str {
  fn render(&self) -> web_sys::Node {
    let text = gloo_utils::document().create_text_node(self);
    text.into()
  }
}

impl RenderNode for &&str {
  fn render(&self) -> web_sys::Node {
    let text = gloo_utils::document().create_text_node(self);
    text.into()
  }
}

impl RenderNode for std::borrow::Cow<'_, str> {
  fn render(&self) -> web_sys::Node {
    let text = gloo_utils::document().create_text_node(self);
    text.into()
  }
}
