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
          path: "/posts",
          element: Promo,
        }
        Route {
          path: "/posts/*p",
          element: Promo,
        }
      }
      Routes {
        Layout {
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
    }
  })
}

#[allow(non_snake_case)]
#[inline_props]
fn Layout<'a>(cx: Scope<'a>, children: Element<'a>) -> Element<'a> {
  cx.render(rsx! {
    NavLink { to: "/", "Home" }
    " | "
    NavLink { to: "/posts", "Posts" }
    div {
      style: "padding: .75rem;",
      children
    }
  })
}

#[allow(non_snake_case)]
fn Home(cx: Scope) -> Element {
  cx.render(rsx! {
    div { "Home" }
  })
}

#[allow(non_snake_case)]
fn Blog(cx: Scope) -> Element {
  cx.render(rsx! {
    div { "Blog" }
    ul {
      li {
        Link {
          to: "/posts/1",
          "Post #1"
        }
      }
    }
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
    div { "Post #1" }
  })
}

#[allow(non_snake_case)]
fn Promo(cx: Scope) -> Element {
  cx.render(rsx! {
    p {
      i {
        "This is a promo shown only in the Blog section"
      }
    }
  })
}
