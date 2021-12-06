use crate::HtmlElement;

use super::{attributes::RenderNodeAttributes, RenderNode};

impl<A, C> RenderNode for HtmlElement<A, C>
where
  A: RenderNodeAttributes,
  C: RenderNode,
{
  fn render(&self) -> web_sys::Node {
    let element = gloo_utils::document()
      .create_element(self.name)
      .expect("Could not create the `{}` element");

    self.attributes.render_attributes_into(&element);

    let node = element.into();
    self.children.render_into(&node);

    node
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
      attributes: HashMap::new(),
      children: Option::<()>::None,
    }
    .render();

    assert_eq!(element.to_string(), "<div></div>");
  }

  #[test]
  fn test_tag_element_with_attributes() {
    let element = HtmlElement {
      name: "div",
      attributes: tuple_list!(("class", "test"), ("id", "test"), ("style", "color: red;"),),
      children: Option::<()>::None,
    }
    .render();

    assert_eq!(
      element.to_string(),
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
        attributes: HashMap::new(),
        children: Some("Hello World"),
      }),
    }
    .render();

    assert_eq!(
      element.to_string(),
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
          attributes: HashMap::new(),
          children: Some(tuple_list!(
            "Hello ",
            HtmlElement {
              name: "span",
              attributes: HashMap::new(),
              children: Some("World"),
            },
          )),
        },
        HtmlElement {
          name: "p",
          attributes: HashMap::new(),
          children: Some("This is a paragraph"),
        },
      )),
    }
    .render();

    assert_eq!(
      element.to_string(),
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
        attributes: HashMap::new(),
        children: Some(vec![
          HtmlElement {
            name: "li",
            attributes: HashMap::new(),
            children: Some("Hello"),
          },
          HtmlElement {
            name: "li",
            attributes: HashMap::new(),
            children: Some("World"),
          },
        ]),
      }),
    }
    .render();

    assert_eq!(
      element.to_string(),
      "<div class=\"test\"><ul><li>Hello</li><li>World</li></ul></div>"
    );
  }
}
