use std::fmt::{Result, Write};

use super::RenderString;
use crate::html::Element;

impl RenderString for Element {
  fn render_into<W: Write>(self, writer: &mut W) -> Result {
    write!(writer, "<{}", self.name)?;
    if !self.attributes.is_empty() {
      self.attributes.render_into(writer)?;
    }
    if self.children.is_empty() {
      let self_closing_tags = [
        "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param",
        "source", "track", "wbr",
      ];

      if self_closing_tags.contains(&self.name) {
        write!(writer, "/>")
      } else {
        write!(writer, "></{}>", self.name)
      }
    } else {
      write!(writer, ">")?;
      self.children.render_into(writer)?;
      write!(writer, "</{}>", self.name)
    }
  }
}

// #[cfg(test)]
// mod test {
//   use crate::html::{AttributeValue, Node};

//   use super::*;

//   #[test]
//   fn test_tag_element() {
//     let element = Element {
//       name: "div",
//       attributes: Default::default(),
//       children: Default::default(),
//     };

//     assert_eq!(element.render(), "<div></div>");
//   }

//   #[test]
//   fn test_tag_element_with_attributes() {
//     let element = Element {
//       name: "div",
//       attributes: vec![
//         (
//           "class".to_owned(),
//           AttributeValue::String("test".to_owned()),
//         ),
//         ("id".to_owned(), AttributeValue::String("test".to_owned())),
//         (
//           "style".to_owned(),
//           AttributeValue::String("color: red;".to_owned()),
//         ),
//       ],
//       children: Default::default(),
//     };

//     assert_eq!(
//       element.render(),
//       "<div class=\"test\" id=\"test\" style=\"color: red;\"></div>"
//     );
//   }

//   #[test]
//   fn test_tag_element_with_one_child() {
//     let element = Element {
//       name: "div",
//       attributes: vec![(
//         "class".to_owned(),
//         AttributeValue::String("test".to_owned()),
//       )],
//       children: vec![Node::Element(Element {
//         name: "h1",
//         attributes: Default::default(),
//         children: vec![Node::Text("Hello World".to_owned())],
//       })],
//     };

//     assert_eq!(
//       element.render(),
//       "<div class=\"test\"><h1>Hello World</h1></div>"
//     );
//   }

//   #[test]
//   fn test_ag_element_with_children() {
//     let element = Element {
//       name: "div",
//       attributes: vec![(
//         "class".to_owned(),
//         AttributeValue::String("test".to_owned()),
//       )],
//       children: vec![
//         Node::Element(Element {
//           name: "h1",
//           attributes: Default::default(),
//           children: vec![
//             Node::Text("Hello ".to_owned()),
//             Node::Element(Element {
//               name: "span",
//               attributes: Default::default(),
//               children: vec![Node::Text("World".to_owned())],
//             }),
//           ],
//         }),
//         Node::Element(Element {
//           name: "p",
//           attributes: Default::default(),
//           children: vec![Node::Text("This is a paragraph".to_owned())],
//         }),
//       ],
//     };

//     assert_eq!(
//       element.render(),
//       "<div class=\"test\"><h1>Hello <span>World</span></h1><p>This is a paragraph</p></div>"
//     );
//   }

//   #[test]
//   fn test_tag_element_with_children_list() {
//     let element = Element {
//       name: "div",
//       attributes: vec![(
//         "class".to_owned(),
//         AttributeValue::String("test".to_owned()),
//       )],
//       children: vec![Node::Element(Element {
//         name: "ul",
//         attributes: Default::default(),
//         children: vec![
//           Node::Element(Element {
//             name: "li",
//             attributes: Default::default(),
//             children: vec![Node::Text("Hello".to_owned())],
//           }),
//           Node::Element(Element {
//             name: "li",
//             attributes: Default::default(),
//             children: vec![Node::Text("World".to_owned())],
//           }),
//         ],
//       })],
//     };

//     assert_eq!(
//       element.render(),
//       "<div class=\"test\"><ul><li>Hello</li><li>World</li></ul></div>"
//     );
//   }
// }

#[cfg(test)]
mod test {
  use crate::{html::Node, Attributes};

  use super::*;

  #[test]
  fn test_tag_element() {
    let element = Element {
      name: "div",
      attributes: Default::default(),
      children: Default::default(),
    };

    assert_eq!(element.render(), "<div></div>");
  }

  #[test]
  fn test_tag_element_with_attributes() {
    let element = Element {
      name: "div",
      attributes: Attributes::default()
        .set("class", "test")
        .set("id", "test")
        .set("style", "color: red;"),
      children: Default::default(),
    };

    assert_eq!(
      element.render(),
      "<div class=\"test\" id=\"test\" style=\"color: red;\"></div>"
    );
  }

  #[test]
  fn test_tag_element_with_one_child() {
    let element = Element {
      name: "div",
      attributes: Attributes::default().set("class", "test"),
      children: vec![Node::Element(Element {
        name: "h1",
        attributes: Default::default(),
        children: vec![Node::Text("Hello World".to_owned())],
      })],
    };

    assert_eq!(
      element.render(),
      "<div class=\"test\"><h1>Hello World</h1></div>"
    );
  }

  #[test]
  fn test_ag_element_with_children() {
    let element = Element {
      name: "div",
      attributes: Attributes::default().set("class", "test"),
      children: vec![
        Node::Element(Element {
          name: "h1",
          attributes: Default::default(),
          children: vec![
            Node::Text("Hello ".to_owned()),
            Node::Element(Element {
              name: "span",
              attributes: Default::default(),
              children: vec![Node::Text("World".to_owned())],
            }),
          ],
        }),
        Node::Element(Element {
          name: "p",
          attributes: Default::default(),
          children: vec![Node::Text("This is a paragraph".to_owned())],
        }),
      ],
    };

    assert_eq!(
      element.render(),
      "<div class=\"test\"><h1>Hello <span>World</span></h1><p>This is a paragraph</p></div>"
    );
  }

  #[test]
  fn test_tag_element_with_children_list() {
    let element = Element {
      name: "div",
      attributes: Attributes::default().set("class", "test"),
      children: vec![Node::Element(Element {
        name: "ul",
        attributes: Default::default(),
        children: vec![
          Node::Element(Element {
            name: "li",
            attributes: Default::default(),
            children: vec![Node::Text("Hello".to_owned())],
          }),
          Node::Element(Element {
            name: "li",
            attributes: Default::default(),
            children: vec![Node::Text("World".to_owned())],
          }),
        ],
      })],
    };

    assert_eq!(
      element.render(),
      "<div class=\"test\"><ul><li>Hello</li><li>World</li></ul></div>"
    );
  }
}
