use std::fmt::{Result, Write};

use crate::escape_html;

pub trait Renderable {
  fn write_attributes<'a, W: Write>(
    &self,
    attributes: &Vec<(&'a str, &'a str)>,
    writer: &mut W,
  ) -> Result {
    if !attributes.is_empty() {
      for (name, value) in attributes {
        write!(writer, " {}=\"", name)?;
        escape_html(&value, writer)?;
        write!(writer, "\"")?;
      }
    }

    Ok(())
  }

  /// Render the component to a writer.
  /// Make sure you escape html correctly using the `render::html_escaping` module
  fn writer<W: Write>(&self, writer: &mut W) -> Result;

  /// Render the component to string
  fn to_string(&self) -> String {
    let mut buf = String::new();
    self.writer(&mut buf).unwrap();
    buf
  }
}

pub trait CustomElement<'a> {
  /// Creates a custom element
  // fn create_custom_element(name: &'a str, attributes: Vec<(&'a str, &'a str)>) -> Self;
  /// The name of the custom element
  fn name(&self) -> &'a str;
  /// The attributes of the custom element
  fn attributes(&self) -> Vec<(&'a str, &'a str)>;
  /// The view of the view of the custom
  fn render(&self) -> Node<'a>;
}

impl<'a> Renderable for dyn CustomElement<'a> {
  fn writer<W: Write>(&self, writer: &mut W) -> Result {
    write!(writer, "<{}", self.name())?;
    self.write_attributes(&self.attributes(), writer)?;
    write!(writer, ">")?;
    self.render().writer(writer)?;
    write!(writer, "</{}>", self.name())
  }
}

pub enum Node<'a> {
  CustomElement(Box<dyn CustomElement<'a>>),
  Empty,
  HtmlElement(Box<HtmlElement<'a>>),
  List(Vec<Node<'a>>),
  Text(&'a str),
}

impl<'a> Renderable for Node<'a> {
  fn writer<W: Write>(&self, writer: &mut W) -> Result {
    match self {
      Node::CustomElement(custom_element) => custom_element.writer(writer),
      Node::Empty => Ok(()),
      Node::HtmlElement(element) => element.writer(writer),
      Node::List(list) => list.writer(writer),
      Node::Text(text) => writer.write_str(text),
    }
  }
}

impl<'a> Renderable for Vec<Node<'a>> {
  fn writer<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
    self.iter().try_for_each(|n| n.writer(writer))
  }
}

pub struct HtmlElement<'a> {
  pub name: &'a str,
  pub attributes: Vec<(&'a str, &'a str)>,
  pub children: Node<'a>,
}

impl<'a> Renderable for HtmlElement<'a> {
  fn writer<W: Write>(&self, writer: &mut W) -> Result {
    match &self.children {
      Node::Empty => {
        write!(writer, "<{}", self.name)?;
        self.write_attributes(&self.attributes, writer)?;
        write!(writer, "/>")
      }
      _ => {
        write!(writer, "<{}", self.name)?;
        self.write_attributes(&self.attributes, writer)?;
        write!(writer, ">")?;
        self.children.writer(writer)?;
        write!(writer, "</{}>", self.name)
      }
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn simple_html_element_test() {
    let element = HtmlElement {
      name: "div",
      attributes: vec![],
      children: Node::Empty,
    };

    assert_eq!(element.to_string(), "<div/>");
  }

  #[test]
  fn simple_html_element_with_attributes_test() {
    let element = HtmlElement {
      name: "div",
      attributes: vec![("class", "test"), ("id", "test"), ("style", "color: red;")],
      children: Node::Empty,
    };

    assert_eq!(
      element.to_string(),
      "<div class=\"test\" id=\"test\" style=\"color: red;\"/>"
    );
  }

  #[test]
  fn simple_html_element_with_children_test() {
    let element = HtmlElement {
      name: "div",
      attributes: vec![("class", "test")],
      children: Node::HtmlElement(Box::new(HtmlElement {
        name: "h1",
        attributes: vec![],
        children: Node::Text("Hello World").into(),
      }))
      .into(),
    };

    assert_eq!(
      element.to_string(),
      "<div class=\"test\"><h1>Hello World</h1></div>"
    );
  }

  #[test]
  fn node_empty_test() {
    let element = Node::Empty;

    assert_eq!(element.to_string(), "");
  }

  #[test]
  fn node_list_test() {
    let element = Node::List(vec![
      Node::Text("Hello ").into(),
      Node::HtmlElement(Box::new(HtmlElement {
        name: "span",
        attributes: vec![],
        children: Node::Text("World").into(),
      }))
      .into(),
    ]);

    assert_eq!(element.to_string(), "Hello <span>World</span>");
  }

  #[test]
  fn node_text_test() {
    let element = Node::Text("Hello World");

    assert_eq!(element.to_string(), "Hello World");
  }

  #[test]
  fn simple_custom_eleemnt_test() {
    struct MyCustomElement {}

    impl<'a> CustomElement<'a> for MyCustomElement {
      fn name(&self) -> &'a str {
        "my-custom-element"
      }

      fn attributes(&self) -> Vec<(&'a str, &'a str)> {
        vec![("class", "test")]
      }

      fn render(&self) -> Node<'a> {
        Node::HtmlElement(Box::new(HtmlElement {
          name: "h1",
          attributes: vec![],
          children: Node::Text("Hello World"),
        }))
      }
    }

    let element = Node::CustomElement(Box::new(MyCustomElement {}));

    assert_eq!(
      element.to_string(),
      "<my-custom-element class=\"test\"><h1>Hello World</h1></my-custom-element>"
    );
  }
}
