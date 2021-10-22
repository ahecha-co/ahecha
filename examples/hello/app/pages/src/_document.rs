use web::Html;

pub struct GlobalStore {}

#[document]
pub struct MainDocument {
  store: GlobalStore,
}

impl Document for MainDocument {
  type Html: ToHtml;
  pub fn render(&self, children: Html) -> Box<dyn ToHtml> {
    html! {
      <html>
        {self.head()}
        <body>
          { children }
        </body>
      </html>
    }
  }
}
