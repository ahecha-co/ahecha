use ahecha_codegen::{document, html};

mod ahecha {
  pub use ahecha_view as view;
}

#[document]
pub fn Document<Head, Body>(
  title: Option<&'static str>,
  head: Head,
  body: Body,
) -> impl ahecha_view::RenderString + '_
where
  Head: ahecha_view::RenderString + 'static,
  Body: ahecha_view::RenderString + 'static,
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
