use crate::{renderable::Renderable, write_attributes, Attributes, Component};

use std::{
  any::{Any, TypeId},
  fmt::{Result, Write},
  rc::Rc,
};

use super::node::Node;

#[derive(Clone)]
pub struct CustomElement<'a> {
  type_id: TypeId,
  component: Rc<dyn Any>,
  pub name: &'a str,
  pub attributes: Attributes<'a>,
  pub children: Option<Box<Node<'a>>>,
}

impl<'a> CustomElement<'a> {
  pub fn new<C>(name: &'a str, attributes: Attributes<'a>, children: Option<Box<Node<'a>>>) -> Self
  where
    C: Component + Default,
  {
    Self {
      type_id: TypeId::of::<C>(),
      component: Rc::new(C::default()),
      name,
      attributes,
      children,
    }
  }
}

impl Renderable for CustomElement<'static> {
  fn writer<W: Write>(&self, writer: &mut W) -> Result {
    if let Ok(component) = self.component.clone().downcast::<&Rc<dyn Component>>() {
      // match &self.children {
      //   None => {
      //     write!(writer, "<{}", self.name)?;
      //     write_attributes(self.attributes.clone(), writer)?;
      //     write!(writer, "/>")
      //   }
      //   Some(children) => {
      //     write!(writer, "<{}", self.name)?;
      //     write_attributes(self.attributes.clone(), writer)?;
      //     write!(writer, ">")?;
      //     component.render().writer(writer)?;
      //     // children.writer(writer)?;
      //     write!(writer, "</{}>", self.name)
      //   }
      // }
      write!(writer, "<{}", self.name)?;
      write_attributes(self.attributes.clone(), writer)?;
      write!(writer, ">")?;
      component.render().writer(writer)?;
      // children.writer(writer)?;
      write!(writer, "</{}>", self.name)
    } else {
      panic!("unexpected component type");
    }
  }
}

impl From<CustomElement<'static>> for String {
  fn from(item: CustomElement<'static>) -> Self {
    item.to_string()
  }
}

impl From<&CustomElement<'static>> for String {
  fn from(item: &CustomElement<'static>) -> Self {
    item.to_string()
  }
}
