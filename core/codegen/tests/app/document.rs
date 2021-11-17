use ahecha_codegen::html;

mod ahecha {
  pub use ahecha_view as view;
}

pub fn Document<Head, Body>(
  title: Option<&str>,
  head: Head,
  body: Body,
) -> impl ahecha_view::Render + '_
where
  Head: ahecha_view::Render + 'static,
  Body: ahecha_view::Render + 'static,
{
  html! {
    <html>
      <head>
        <title>{ title.unwrap_or("") }</title>
        { head }
      </head>
      <body>
        { body }
      </body>
    </html>
  }
}
