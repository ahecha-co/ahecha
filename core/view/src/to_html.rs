use std::fmt::{Result, Write};

pub trait ToHtml: Sized {
  /// Render the component to a writer.
  /// Make sure you escape html correctly using the `render::html_escaping` module
  fn html_into<W: Write>(self, writer: &mut W) -> Result;

  /// Render the component to string
  fn to_html(self) -> String {
    let mut buf = String::new();
    self.html_into(&mut buf).unwrap();
    buf
  }
}

/// Does nothing
impl ToHtml for () {
  fn html_into<W: Write>(self, _writer: &mut W) -> Result {
    Ok(())
  }
}

/// Renders string
impl ToHtml for &str {
  fn html_into<W: Write>(self, writer: &mut W) -> Result {
    write!(writer, "{}", self)
  }
}

/// Renders string
impl ToHtml for String {
  fn html_into<W: Write>(self, writer: &mut W) -> Result {
    write!(writer, "{}", self)
  }
}

/// Renders `A`, then `B`
impl<A: ToHtml, B: ToHtml> ToHtml for (A, B) {
  fn html_into<W: Write>(self, writer: &mut W) -> Result {
    self.0.html_into(writer)?;
    self.1.html_into(writer)
  }
}

/// Renders `A`, then `B`, then `C`
impl<A: ToHtml, B: ToHtml, C: ToHtml> ToHtml for (A, B, C) {
  fn html_into<W: Write>(self, writer: &mut W) -> Result {
    self.0.html_into(writer)?;
    self.1.html_into(writer)?;
    self.2.html_into(writer)
  }
}

/// Renders `T` or nothing
impl<T: ToHtml> ToHtml for Option<T> {
  fn html_into<W: Write>(self, writer: &mut W) -> Result {
    match self {
      None => Ok(()),
      Some(x) => x.html_into(writer),
    }
  }
}

impl<T: ToHtml> ToHtml for Vec<T> {
  fn html_into<W: Write>(self, writer: &mut W) -> Result {
    for elem in self {
      elem.html_into(writer)?;
    }
    Ok(())
  }
}

/// Renders `O` or `E`
impl<O: ToHtml, E: ToHtml> ToHtml for std::result::Result<O, E> {
  fn html_into<W: Write>(self, writer: &mut W) -> Result {
    match self {
      Ok(o) => o.html_into(writer),
      Err(e) => e.html_into(writer),
    }
  }
}
