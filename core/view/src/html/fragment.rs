use std::fmt::{Result, Write};

use crate::html::render::Render;

pub struct HtmlFragment<C>
where
  C: Render,
{
  pub children: Option<C>,
}

impl<C> Render for HtmlFragment<C>
where
  C: Render,
{
  fn render_into<W: Write>(self, writer: &mut W) -> Result {
    if let Some(children) = self.children {
      children.render_into(writer)
    } else {
      Ok(())
    }
  }
}

impl<C> From<HtmlFragment<C>> for String
where
  C: Render,
{
  fn from(element: HtmlFragment<C>) -> Self {
    element.render()
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
    };

    assert_eq!(element.render(), "");
  }

  #[test]
  fn test_fragment_with_text() {
    let element = HtmlFragment {
      children: Some("I'm a fragment"),
    };

    assert_eq!(element.render(), "I&apos;m a fragment");
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
    };

    assert_eq!(element.render(), "<div>I&apos;m a fragment</div>");
  }
}
