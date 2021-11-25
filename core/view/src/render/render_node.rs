mod attributes;
mod doctype;
mod elements;
mod fragment;
mod numbers;
mod text;
mod tuples;

pub trait RenderNode {
  fn render_into(&self, parent: &web_sys::Node) {
    parent
      .append_child(&self.render())
      .expect("Failed to append child");
  }

  fn render(&self) -> web_sys::Node;
}

/// IntoHtmlElements `()` or nothing
impl RenderNode for () {
  fn render_into(&self, _parent: &web_sys::Node) {}

  fn render(&self) -> web_sys::Node {
    unimplemented!()
  }
}

/// IntoHtmlElements `T` or nothing
impl<T: RenderNode> RenderNode for Option<T> {
  fn render_into(&self, parent: &web_sys::Node) {
    match self {
      None => {}
      Some(x) => x.render_into(parent),
    }
  }

  fn render(&self) -> web_sys::Node {
    unimplemented!()
  }
}

/// IntoHtmlElements a list of `T`
impl<T: RenderNode> RenderNode for Vec<T> {
  fn render_into(&self, parent: &web_sys::Node) {
    self.iter().for_each(|child| child.render_into(parent));
  }

  // We cannot generate a node from a list of elements, use append_to instead
  fn render(&self) -> web_sys::Node {
    unimplemented!()
  }
}

// /// IntoHtmlElements `O` or `E`
// impl<O: IntoHtmlElement, E: IntoHtmlElement> IntoHtmlElement for std::result::Result<O, E> {
//   fn into_html_element(&self) -> Result<web_sys::Node> {
//     match self {
//       Ok(o) => o.render_into(writer),
//       Err(e) => e.render_into(writer),
//     }
//   }
// }

/// IntoHtmlElements `bool`
impl RenderNode for bool {
  fn render(&self) -> web_sys::Node {
    let text = gloo_utils::document().create_text_node(if *self { "true" } else { "false" });
    text.into()
  }
}
