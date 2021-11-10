use std::fmt::{Result, Write};

/// Simple HTML escaping, so strings can be safely rendered.
///
/// ```rust
/// //# use html_escaping;
///
/// // let mut buf = String::new();
/// // escape_html(r#"<hello world="attribute" />"#, &mut buf).unwrap();
/// // assert_eq!(buf, "&lt;hello world=&quot;attribute&quot; /&gt;");
/// ```
pub fn escape_html<W: Write>(html: &str, writer: &mut W) -> Result {
  for c in html.chars() {
    match c {
      '>' => write!(writer, "&gt;")?,
      '<' => write!(writer, "&lt;")?,
      '"' => write!(writer, "&quot;")?,
      '&' => write!(writer, "&amp;")?,
      '\'' => write!(writer, "&apos;")?,
      c => writer.write_char(c)?,
    };
  }

  Ok(())
}
