use ahecha_tuple_list::TupleList;

pub trait RenderNodeAttributeValue {
  fn to_attribute_value(&self) -> String;
}

impl RenderNodeAttributeValue for Option<&str> {
  fn to_attribute_value(&self) -> String {
    match self {
      Some(s) => s.to_string(),
      None => "".to_string(),
    }
  }
}

macro_rules! impl_attribute_value {
  ($($t:ty),*) => {
    $(impl RenderNodeAttributeValue for $t {
      fn to_attribute_value(&self) -> String {
        self.to_string()
      }
    })*
  };
}

impl_attribute_value!(&str, String, bool, u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);

pub trait RenderNodeAttributes {
  fn render_attributes_into(&self, element: &web_sys::Element);
}

impl RenderNodeAttributes for () {
  fn render_attributes_into(&self, _element: &web_sys::Element) {}
}

impl<A> RenderNodeAttributes for (&str, A)
where
  A: RenderNodeAttributeValue,
{
  fn render_attributes_into(&self, element: &web_sys::Element) {
    element
      .set_attribute(self.0, self.1.to_attribute_value().as_str())
      .expect(
        format!(
          "To set attribute `{}`=\"{}\"",
          self.0,
          self.1.to_attribute_value()
        )
        .as_str(),
      );
  }
}

impl<A, Tail> RenderNodeAttributes for ((&str, A), Tail)
where
  A: RenderNodeAttributeValue,
  Tail: RenderNodeAttributes + TupleList,
{
  fn render_attributes_into(&self, element: &web_sys::Element) {
    self.0.render_attributes_into(&element);
    self.1.render_attributes_into(&element);
  }
}
