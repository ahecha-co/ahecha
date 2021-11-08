use std::fmt::{Result, Write};

use crate::backend::{attributes::RenderAttributes, render::Render};

use super::HtmlElement;

pub struct TagElement<A, C>
where
  A: RenderAttributes,
  C: Render,
{
  pub name: String,
  pub attributes: A,
  pub children: Option<C>,
}

impl<A, C> HtmlElement<A, C> for TagElement<A, C>
where
  A: RenderAttributes,
  C: Render,
{
  fn new(name: &str, attributes: A, children: Option<C>) -> Self {
    Self {
      name: name.into(),
      attributes,
      children,
    }
  }

  fn attributes(&self) -> &A {
    &self.attributes
  }
}

impl<A, C> Render for TagElement<A, C>
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

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_tag_element() {
    let element = TagElement::new("div", (), ().into());

    assert_eq!(element.to_string(), "<div></div>");
  }

  #[test]
  fn test_tag_element_with_attributes() {
    let element = TagElement::new(
      "div",
      vec![("class", "test"), ("id", "test"), ("style", "color: red;")],
      ().into(),
    );

    assert_eq!(
      element.to_string(),
      "<div class=\"test\" id=\"test\" style=\"color: red;\"></div>"
    );
  }

  #[test]
  fn test_tag_element_with_one_child() {
    let element = TagElement::new(
      "div",
      ("class", "test"),
      TagElement::new("h1", (), "Hello World".into()).into(),
    );

    assert_eq!(
      element.to_string(),
      "<div class=\"test\"><h1>Hello World</h1></div>"
    );
  }

  #[test]
  fn test_ag_element_with_children() {
    let element = TagElement::new(
      "div",
      ("class", "test"),
      (
        TagElement::new(
          "h1",
          (),
          ("Hello ", TagElement::new("span", (), "World".into())).into(),
        ),
        TagElement::new("p", (), "This is a paragraph".into()),
      )
        .into(),
    );

    assert_eq!(
      element.to_string(),
      "<div class=\"test\"><h1>Hello <span>World</span></h1><p>This is a paragraph</p></div>"
    );
  }

  #[test]
  fn test_tag_element_with_children_list() {
    let element = TagElement::new(
      "div",
      ("class", "test"),
      TagElement::new(
        "ul",
        (),
        vec![
          TagElement::new("li", (), "Hello".into()),
          TagElement::new("li", (), "World".into()),
        ]
        .into(),
      )
      .into(),
    );

    assert_eq!(
      element.to_string(),
      "<div class=\"test\"><ul><li>Hello</li><li>World</li></ul></div>"
    );
  }
}
