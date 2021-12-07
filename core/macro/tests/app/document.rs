use ahecha_macro::{document, html};

mod ahecha {
  pub use ahecha_html as html;
}

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
