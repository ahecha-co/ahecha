use etagere::prelude::*;

#[component]
pub fn PostComponent<'a>(title: &'a str, body: &'a str, image: Option<&'a str>) {
  html! {
    <div class="px-4 py-5 my-5 text-center">
      <h1 class="display-5 fw-bold">{ title }</h1>
      <div class="col-lg-6 mx-auto">
        <p class="lead mb-4">{ body }</p>
        { if let Some(image) = image {
          html! {
            <div class="d-grid gap-2 d-sm-flex justify-content-sm-center">
              <img src={ image } />
            </div>
          }
        } else {
          None
        }}
      </div>
    </div>
  }
}
