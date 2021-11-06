use std::{
  collections::HashMap,
  fmt::{Result, Write},
};

use crate::backend::render::Render;

use super::HtmlElement;

#[derive(Default)]
pub struct TagElement<T>
where
  T: Render + Default,
{
  pub name: String,
  pub attributes: std::collections::HashMap<String, String>,
  pub children: Option<T>,
}

impl<T> HtmlElement<T> for TagElement<T>
where
  T: Render + Default,
{
  type Attributes = HashMap<String, String>;

  fn create(&mut self, name: String, attributes: Self::Attributes, children: Option<T>) {
    self.name = name;
    self.attributes = attributes;
    self.children = children;
  }

  fn attributes(&self) -> Self::Attributes {
    self.attributes.clone()
  }
}

impl<T> Render for TagElement<T>
where
  T: Render + Default,
{
  fn render_into<W: Write>(self, writer: &mut W) -> Result {
    match self.children {
      None => {
        write!(writer, "<{}", self.name)?;
        self.attributes.render_into(writer)?;
        write!(writer, "/>")
      }
      Some(children) => {
        write!(writer, "<{}", self.name)?;
        self.attributes.render_into(writer)?;
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
    let element = TagElement::<String> {
      name: "div".into(),
      ..Default::default()
    };

    assert_eq!(element.to_string(), "<div/>");
  }

  #[test]
  fn test_tag_element_with_attributes() {
    let element = TagElement::<String> {
      name: "div".into(),
      attributes: [
        ("class".into(), "test".into()),
        ("id".into(), "test".into()),
        ("style".into(), "color: red;".into()),
      ]
      .into(),
      ..Default::default()
    };

    assert_eq!(
      element.to_string(),
      "<div class=\"test\" id=\"test\" style=\"color: red;\"/>"
    );
  }

  #[test]
  fn test_tag_element_with_one_child() {
    let element = TagElement {
      name: "div".into(),
      attributes: [("class".into(), "test".into())].into(),
      children: TagElement {
        name: "h1".into(),
        attributes: Default::default(),
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
    let element = TagElement {
      name: "div".into(),
      attributes: [("class".into(), "test".into())].into(),
      children: (
        TagElement {
          name: "h1".into(),
          attributes: Default::default(),
          children: (
            "Hello ",
            TagElement {
              name: "span".into(),
              attributes: Default::default(),
              children: "World".into(),
            },
          )
            .into(),
        },
        TagElement {
          name: "p".into(),
          attributes: Default::default(),
          children: "Loren ipsum".into(),
        },
      )
        .into(),
    };

    assert_eq!(
      element.to_string(),
      "<div class=\"test\"><h1>Hello <span>World</span></h1><p>Loren ipsum</p></div>"
    );
  }

  #[test]
  fn test_tag_element_with_children_list() {
    let element = TagElement {
      name: "div".into(),
      attributes: [("class".into(), "test".into())].into(),
      children: TagElement {
        name: "ul".into(),
        attributes: Default::default(),
        children: vec![
          TagElement {
            name: "li".into(),
            attributes: Default::default(),
            children: "Hello".into(),
          },
          TagElement {
            name: "li".into(),
            attributes: Default::default(),
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

  #[test]
  fn test_tag_element_with_children_tuple() {
    let element = TagElement {
      name: "div".into(),
      attributes: [("class".into(), "test".into())].into(),
      children: TagElement {
        name: "ul".into(),
        attributes: Default::default(),
        children: (
          TagElement {
            name: "li".into(),
            attributes: Default::default(),
            children: "1".into(),
          },
          TagElement {
            name: "li".into(),
            attributes: Default::default(),
            children: "2".into(),
          },
          TagElement {
            name: "li".into(),
            attributes: Default::default(),
            children: "3".into(),
          },
          TagElement {
            name: "li".into(),
            attributes: Default::default(),
            children: "4".into(),
          },
          TagElement {
            name: "li".into(),
            attributes: Default::default(),
            children: "5".into(),
          },
        )
          .into(),
      }
      .into(),
    };

    assert_eq!(
      element.to_string(),
      "<div class=\"test\"><ul><li>1</li><li>2</li><li>3</li><li>4</li><li>5</li></ul></div>"
    );
  }
}
