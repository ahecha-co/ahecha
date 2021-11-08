use crate::Render;

use super::attributes::RenderAttributes;

mod numbers;
mod tag;
mod text;
mod tuples;

pub trait HtmlElement<A, C>
where
  A: RenderAttributes,
  C: Render,
{
  /// Set the initial values of the custom element, this is called when creating the element
  fn new(name: &str, attributes: A, children: Option<C>) -> Self;

  /// The attributes of the custom element
  fn attributes(&self) -> &A;

  /// The view of the view of the custom
  fn render(&self) -> Option<C> {
    None
  }
}

#[cfg(test)]
mod tests {
  use super::{tag::TagElement, *};

  #[test]
  fn test_html_element() {
    let element = TagElement::new(
      "div",
      (),
      (
        "Hello, Block!",
        TagElement::new(
          "ul",
          ("class", "list"),
          [1, 2, 3]
            .iter()
            .map(|i| TagElement::new("li", (), (*i).into()))
            .collect::<Vec<_>>()
            .into(),
        ),
      )
        .into(),
    );

    assert_eq!(
      element.to_string(),
      "<div>Hello, Block!<ul class=\"list\"><li>1</li><li>2</li><li>3</li></ul></div>"
    );
  }
}
