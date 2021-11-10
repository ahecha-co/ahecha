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

// impl<A, C> Into<String> for HtmlElement<A, C>
// where
//   A: RenderAttributes,
//   C: Render,
// {
//   fn into(self) -> String {
//     let mut result = String::new();
//     self.render_into(&mut result).unwrap();
//     result
//   }
// }

impl<A, C> From<HtmlElement<A, C>> for String
where
  A: RenderAttributes,
  C: Render,
{
  fn from(element: HtmlElement<A, C>) -> Self {
    element.render()
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
      children: Option::<()>::None,
    };

    assert_eq!(element.render(), "<div/>");
  }

  #[test]
  fn test_tag_element_with_attributes() {
    let element = HtmlElement {
      name: "div",
      attributes: tuple_list::tuple_list!(
        ("class", "test"),
        ("id", "test"),
        ("style", "color: red;"),
      ),
      children: Option::<()>::None,
    };

    assert_eq!(
      element.render(),
      "<div class=\"test\" id=\"test\" style=\"color: red;\"/>"
    );
  }

  #[test]
  fn test_tag_element_with_one_child() {
    let element = HtmlElement {
      name: "div",
      attributes: (("class", "test"), ()),
      children: Some(HtmlElement {
        name: "h1",
        attributes: (),
        children: Some("Hello World"),
      }),
    };

    assert_eq!(
      element.render(),
      "<div class=\"test\"><h1>Hello World</h1></div>"
    );
  }

  #[test]
  fn test_ag_element_with_children() {
    let element = HtmlElement {
      name: "div",
      attributes: (("class", "test"), ()),
      children: Some(tuple_list::tuple_list!(
        HtmlElement {
          name: "h1",
          attributes: (),
          children: Some(tuple_list::tuple_list!(
            "Hello ",
            HtmlElement {
              name: "span",
              attributes: (),
              children: Some("World"),
            },
          )),
        },
        HtmlElement {
          name: "p",
          attributes: (),
          children: Some("This is a paragraph"),
        },
      )),
    };

    assert_eq!(
      element.render(),
      "<div class=\"test\"><h1>Hello <span>World</span></h1><p>This is a paragraph</p></div>"
    );
  }

  #[test]
  fn test_tag_element_with_children_list() {
    let element = HtmlElement {
      name: "div",
      attributes: (("class", "test"), ()),
      children: Some(HtmlElement {
        name: "ul",
        attributes: (),
        children: Some(vec![
          HtmlElement {
            name: "li",
            attributes: (),
            children: Some("Hello"),
          },
          HtmlElement {
            name: "li",
            attributes: (),
            children: Some("World"),
          },
        ]),
      }),
    };

    assert_eq!(
      element.render(),
      "<div class=\"test\"><ul><li>Hello</li><li>World</li></ul></div>"
    );
  }
}
