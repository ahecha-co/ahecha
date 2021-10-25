use std::fmt::{Result, Write};

pub trait Renderable: Sized {
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
