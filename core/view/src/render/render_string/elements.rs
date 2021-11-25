use std::fmt::{Result, Write};

use crate::HtmlElement;

use super::{attributes::RenderAttributes, RenderString};

impl<A, C> RenderString for HtmlElement<A, C>
where
  A: RenderAttributes,
  C: RenderString,
{
  fn render_into<W: Write>(&self, writer: &mut W) -> Result {
    write!(writer, "<{}", self.name)?;
    self.attributes.render_attributes_into(writer)?;

    match &self.children {
      None => {
        let self_closing_tags = [
          "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param",
          "source", "track", "wbr",
        ];

        if self_closing_tags.contains(&self.name) {
          write!(writer, "/>")
        } else {
          write!(writer, "></{}>", self.name)
        }
      }
      Some(children) => {
        write!(writer, ">")?;
        children.render_into(writer)?;
        write!(writer, "</{}>", self.name)
      }
    }
  }
}

impl<A, C> ToString for HtmlElement<A, C>
where
  A: RenderAttributes,
  C: RenderString,
{
  fn to_string(&self) -> String {
    self.render()
  }
}

#[cfg(test)]
mod test {
  use ahecha_tuple_list::tuple_list;

  use super::*;

  #[test]
  fn test_tag_element() {
    let element = HtmlElement {
      name: "div",
      attributes: (),
      children: Option::<()>::None,
    };

    assert_eq!(element.render(), "<div></div>");
  }

  #[test]
  fn test_tag_element_with_attributes() {
    let element = HtmlElement {
      name: "div",
      attributes: tuple_list!(("class", "test"), ("id", "test"), ("style", "color: red;"),),
      children: Option::<()>::None,
    };

    assert_eq!(
      element.render(),
      "<div class=\"test\" id=\"test\" style=\"color: red;\"></div>"
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
      children: Some(tuple_list!(
        HtmlElement {
          name: "h1",
          attributes: (),
          children: Some(tuple_list!(
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
