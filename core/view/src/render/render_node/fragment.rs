use crate::HtmlFragment;

use super::RenderNode;

impl<C> RenderNode for HtmlFragment<C>
where
  C: RenderNode,
{
  fn render_into(&self, parent: &web_sys::Node) {
    self.children.render_into(&parent);
  }

  fn render(&self) -> web_sys::Node {
    unimplemented!()
  }
}

#[cfg(test)]
mod test {
  use crate::HtmlElement;

  use super::*;

  #[test]
  fn test_fragment() {
    let element = HtmlFragment {
      children: Option::<()>::None,
    }
    .render();

    assert_eq!(element.to_string(), "");
  }

  #[test]
  fn test_fragment_with_text() {
    let element = HtmlFragment {
      children: Some("I'm a fragment"),
    }
    .render();

    assert_eq!(element.to_string(), "I&apos;m a fragment");
  }

  #[test]
  fn test_fragment_with_children() {
    let element = HtmlFragment {
      children: Some(HtmlElement {
        name: "div",
        attributes: (),
        children: Some(HtmlFragment {
          children: "I'm a fragment".into(),
        }),
      }),
    }
    .render();

    assert_eq!(element.to_string(), "<div>I&apos;m a fragment</div>");
  }
}
