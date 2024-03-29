use dioxus::prelude::*;

#[::ahecha::page]
#[allow(non_snake_case)]
pub fn Index(cx: Scope) -> Element {
  cx.render(rsx!(
    h1 {
      class: "text-gray-900 text-xs",
      "Hello world!"
    }
    p {
      class: "text-gray-700 text-sm",
      "Hello world from router"
    }
  ))
}
