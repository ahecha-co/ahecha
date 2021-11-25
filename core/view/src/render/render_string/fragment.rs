use std::fmt::{Result, Write};

use crate::HtmlFragment;

use super::RenderString;

impl<C> RenderString for HtmlFragment<C>
where
  C: RenderString,
{
  fn render_into<W: Write>(&self, writer: &mut W) -> Result {
    if let Some(children) = &self.children {
      children.render_into(writer)
    } else {
      Ok(())
    }
  }
}

impl<C> ToString for HtmlFragment<C>
where
  C: RenderString,
{
  fn to_string(&self) -> String {
    self.render()
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::HtmlElement;

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
