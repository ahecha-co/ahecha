use std::{
  collections::HashMap,
  fmt::{Result, Write},
};

use crate::escape_html;

pub trait Render: Sized {
  /// Render the component to a writer.
  /// Make sure you escape html correctly using the `render::html_escaping` module
  fn render_into<W: Write>(self, writer: &mut W) -> Result;

  /// Render the component to string
  fn to_string(self) -> String {
    let mut buf = String::new();
    self.render_into(&mut buf).unwrap();
    buf
  }
}

/// Renders a hashmap as html attributes
impl Render for HashMap<String, String> {
  fn render_into<W: Write>(self, writer: &mut W) -> Result {
    let mut keys_values = self.iter().collect::<Vec<(&String, &String)>>();
    keys_values.sort_by(|a, b| a.0.cmp(b.0));

    for (key, value) in keys_values.iter() {
      write!(writer, " {}=\"", key)?;
      escape_html(value, writer)?;
      write!(writer, "\"")?;
    }
    Ok(())
  }
}

/// Renders `T` or nothing
impl<T: Render> Render for Option<T> {
  fn render_into<W: Write>(self, writer: &mut W) -> Result {
    match self {
      None => Ok(()),
      Some(x) => x.render_into(writer),
    }
  }
}

/// Renders a list of `T`
impl<T: Render> Render for Vec<T> {
  fn render_into<W: Write>(self, writer: &mut W) -> Result {
    for elem in self {
      elem.render_into(writer)?;
    }
    Ok(())
  }
}

/// Renders `O` or `E`
impl<O: Render, E: Render> Render for std::result::Result<O, E> {
  fn render_into<W: Write>(self, writer: &mut W) -> Result {
    match self {
      Ok(o) => o.render_into(writer),
      Err(e) => e.render_into(writer),
    }
  }
}

/// Renders `bool`
impl Render for bool {
  fn render_into<W: Write>(self, writer: &mut W) -> Result {
    if self {
      write!(writer, "true")?;
    } else {
      write!(writer, "false")?;
    }
    Ok(())
  }
}