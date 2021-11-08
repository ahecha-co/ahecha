mod numbers;
mod tag;
mod text;
mod tuples;

pub trait HtmlElement<T>: Default {
  type Attributes: Default + Clone;
  /// Set the initial values of the custom element, this is called when creating the element
  fn create(&mut self, name: String, attributes: Self::Attributes, children: Option<T>);

  /// The attributes of the custom element
  fn attributes(&self) -> Self::Attributes;

  /// The view of the view of the custom
  fn render(&self) -> Option<T> {
    None
  }
}

#[cfg(test)]
mod tests {
  use super::{tag::TagElement, *};
  use crate::backend::render::Render;

  #[test]
  fn test_html_element() {
    let element = TagElement {
      name: "div".into(),
      attributes: Default::default(),
      children: (
        "Hello, Block!",
        TagElement {
          name: "ul".into(),
          attributes: [("class".into(), "list".into())].iter().cloned().collect(),
          children: [1, 2, 3]
            .iter()
            .map(|i| TagElement {
              name: "li".into(),
              attributes: Default::default(),
              children: (*i).into(),
            })
            .collect::<Vec<_>>()
            .into(),
        },
      )
        .into(),
    };

    assert_eq!(
      element.to_string(),
      "<div>Hello, Block!<ul class=\"list\"><li>1</li><li>2</li><li>3</li></ul></div>"
    );
  }
}
