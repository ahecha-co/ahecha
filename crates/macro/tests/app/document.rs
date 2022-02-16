use ahecha::prelude::*;

#[document]
pub fn Document(
  title: Option<&'static str>,
  head: ahecha::html::Node,
  body: ahecha::html::Node,
) -> ahecha::html::Node {
  html! {
    <html>
      <head>
        <title>{ title.unwrap_or("Document title") }</title>
        { head }
      </head>
      <body>
        { body }
      </body>
    </html>
  }
}
