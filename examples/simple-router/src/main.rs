use ahecha::*;
use dioxus::prelude::*;

fn main() {
  wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
  tracing_wasm::set_as_global_default();
  dioxus_web::launch(app);
}

fn app(cx: Scope) -> Element {
  cx.render(rsx! {
    BrowserRouter {
      Routes {
        Route {
          path: "/",
          element: Home,
        }
        Route {
          path: "/posts",
          element: Blog,
          Route {
            path: ":id"
            element: Post
          }
        }
        Fallback {
          element: NotFound,
        }
      }
    }
  })
}

#[allow(non_snake_case)]
fn Home(cx: Scope) -> Element {
  cx.render(rsx! {
    Link { to: "/posts", "Posts" }
    div { "Home" }
  })
}

#[allow(non_snake_case)]
fn Blog(cx: Scope) -> Element {
  cx.render(rsx! {
    Link { to: "/", "Home" }
    div { "Blog" }
  })
}

#[allow(non_snake_case)]
fn NotFound(cx: Scope) -> Element {
  cx.render(rsx! {
    div { "Not Found" }
  })
}

#[allow(non_snake_case)]
fn Post(cx: Scope) -> Element {
  cx.render(rsx! {
    div { "Post" }
  })
}
