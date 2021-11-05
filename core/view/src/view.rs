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
  /// Set the initial values of the custom element, this is called when creating the element
  fn create(&mut self, _attributes: Vec<(&'a str, &'a str)>, _children: Node<'a>) {}
  /// The attributes of the custom element
  fn attributes(&self) -> Vec<(&'a str, &'a str)> {
    vec![]
  }
  /// The view of the view of the custom
  fn render(&self) -> Node<'a>;
}

pub enum Node<'a> {
  CustomElement(Box<CustomElementWrapper<'a>>),
  None,
  HtmlElement(Box<HtmlElement<'a>>),
  List(Vec<Node<'a>>),
  Text(&'a str),
}

impl<'a> Default for Node<'a> {
  fn default() -> Self {
    Node::None
  }
}

impl<'a> Into<String> for Node<'a> {
  fn into(self) -> String {
    match self {
      Node::CustomElement(custom) => custom.to_string(),
      Node::None => "".to_string(),
      Node::HtmlElement(element) => element.to_string(),
      Node::List(list) => list
        .into_iter()
        .map(|node| node.to_string())
        .collect::<Vec<String>>()
        .join(""),
      Node::Text(text) => text.to_string(),
    }
  }
}

impl<'a> Renderable for Node<'a> {
  fn writer<W: Write>(&self, writer: &mut W) -> Result {
    match self {
      Node::CustomElement(custom_element) => custom_element.writer(writer),
      Node::None => Ok(()),
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

pub struct CustomElementWrapper<'a> {
  pub name: &'a str,
  pub custom_element: Box<dyn CustomElement<'a>>,
}

impl<'a> Renderable for CustomElementWrapper<'a> {
  fn writer<W: Write>(&self, writer: &mut W) -> Result {
    write!(writer, "<{}", self.name)?;
    self.write_attributes(&self.custom_element.attributes(), writer)?;
    write!(writer, ">")?;
    self.custom_element.render().writer(writer)?;
    write!(writer, "</{}>", self.name)
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
      Node::None => {
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
      children: Node::None,
    };

    assert_eq!(element.to_string(), "<div/>");
  }

  #[test]
  fn simple_html_element_with_attributes_test() {
    let element = HtmlElement {
      name: "div",
      attributes: vec![("class", "test"), ("id", "test"), ("style", "color: red;")],
      children: Node::None,
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
    let element = Node::None;

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
    #[derive(Default)]
    struct MyCustomElement<'a> {
      attributes: Vec<(&'a str, &'a str)>,
      children: Node<'a>,
    }

    impl<'a> CustomElement<'a> for MyCustomElement<'a> {
      fn create(&mut self, attributes: Vec<(&'a str, &'a str)>, children: Node<'a>) {
        self.attributes = attributes;
        self.children = children;
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

    let element = Node::CustomElement(Box::new(CustomElementWrapper {
      name: "my-custom-element",
      custom_element: Box::new({
        let mut element = MyCustomElement::default();
        element.create(vec![("class", "test")], Node::Text("Hello World"));
        element
      }),
    }));

    assert_eq!(
      element.to_string(),
      "<my-custom-element class=\"test\"><h1>Hello World</h1></my-custom-element>"
    );
  }
}
