use ita::prelude::*;

use components::PostComponent;

#[component]
pub fn IndexPage() {
  html! {
    <html lang="en">
      <body>
        <h1>"Ita blog example"</h1>
        <PostComponent title="Hello, world!" body="This is the first post." image={Some("https://cataas.com/cat")} />
      </body>
    </html>
  }
}
