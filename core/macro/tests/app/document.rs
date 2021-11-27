use ahecha_macro::{document, html};

mod ahecha {
  pub use ahecha_html as view;
}

#[document]
pub fn Document<Head, Body>(
  title: Option<&'static str>,
  head: Head,
  body: Body,
) -> impl ahecha_html::RenderString + '_
where
  Head: ahecha_html::RenderString + 'static,
  Body: ahecha_html::RenderString + 'static,
{
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
