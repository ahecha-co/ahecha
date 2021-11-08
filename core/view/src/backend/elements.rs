mod numbers;
mod text;
mod tuples;

use std::fmt::{Result, Write};

use crate::backend::{attributes::RenderAttributes, render::Render};

pub struct HtmlElement<A, C>
where
  A: RenderAttributes,
  C: Render,
{
  pub name: &'static str,
  pub attributes: A,
  pub children: Option<C>,
}

impl<A, C> Render for HtmlElement<A, C>
where
  A: RenderAttributes,
  C: Render,
{
  fn render_into<W: Write>(self, writer: &mut W) -> Result {
    write!(writer, "<{}", self.name)?;
    self.attributes.render_attributes_into(writer)?;

    match self.children {
      None => {
        write!(writer, "/>")
      }
      Some(children) => {
        write!(writer, ">")?;
        children.render_into(writer)?;
        write!(writer, "</{}>", self.name)
      }
    }
  }
}

impl<A, C> Into<String> for HtmlElement<A, C>
where
  A: RenderAttributes,
  C: Render,
{
  fn into(self) -> String {
    let mut result = String::new();
    self.render_into(&mut result).unwrap();
    result
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_tag_element() {
    let element = HtmlElement {
      name: "div",
      attributes: (),
      children: ().into(),
    };

    assert_eq!(element.to_string(), "<div></div>");
  }

  #[test]
  fn test_tag_element_with_attributes() {
    let element = HtmlElement {
      name: "div",
      attributes: vec![("class", "test"), ("id", "test"), ("style", "color: red;")],
      children: ().into(),
    };

    assert_eq!(
      element.to_string(),
      "<div class=\"test\" id=\"test\" style=\"color: red;\"></div>"
    );
  }

  #[test]
  fn test_tag_element_with_one_child() {
    let element = HtmlElement {
      name: "div",
      attributes: (("class", "test"),),
      children: HtmlElement {
        name: "h1",
        attributes: (),
        children: "Hello World".into(),
      }
      .into(),
    };

    assert_eq!(
      element.to_string(),
      "<div class=\"test\"><h1>Hello World</h1></div>"
    );
  }

  #[test]
  fn test_ag_element_with_children() {
    let element = HtmlElement {
      name: "div",
      attributes: (("class", "test"),),
      children: (
        HtmlElement {
          name: "h1",
          attributes: (),
          children: (
            "Hello ",
            HtmlElement {
              name: "span",
              attributes: (),
              children: "World".into(),
            },
          )
            .into(),
        },
        HtmlElement {
          name: "p",
          attributes: (),
          children: "This is a paragraph".into(),
        },
      )
        .into(),
    };

    assert_eq!(
      element.to_string(),
      "<div class=\"test\"><h1>Hello <span>World</span></h1><p>This is a paragraph</p></div>"
    );
  }

  #[test]
  fn test_tag_element_with_children_list() {
    let element = HtmlElement {
      name: "div",
      attributes: (("class", "test"),),
      children: HtmlElement {
        name: "ul",
        attributes: (),
        children: vec![
          HtmlElement {
            name: "li",
            attributes: (),
            children: "Hello".into(),
          },
          HtmlElement {
            name: "li",
            attributes: (),
            children: "World".into(),
          },
        ]
        .into(),
      }
      .into(),
    };

    assert_eq!(
      element.to_string(),
      "<div class=\"test\"><ul><li>Hello</li><li>World</li></ul></div>"
    );
  }
}
