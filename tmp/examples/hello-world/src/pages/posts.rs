use dioxus::prelude::*;

#[::router::page]
#[allow(non_snake_case)]
pub fn Posts(cx: Scope) -> Element {
  cx.render(rsx!(
    h1 {
      class: "text-gray-900 text-xs",
      "Posts"
    }
    p {
      class: "text-gray-700 text-sm",
      "Posts from router"
    }
  ))
}
