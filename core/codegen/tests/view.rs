use etagere_codegen::*;

mod etagere {
  pub use etagere_view as view;
}

#[test]
fn test_html_tag() {
  let res: String = html! { <div></div> };
  assert_eq!(res, "<div/>");
}

#[test]
fn test_html_tag_with_text() {
  let res: String = html! { <div>"Text"</div> };
  assert_eq!(res, "<div>Text</div>");
}

#[test]
fn test_html_tag_with_attributes() {
  let res: String = html! { <div class="some_class">"Text"</div> };
  assert_eq!(res, "<div class=\"some_class\">Text</div>");
}

#[test]
fn test_html_with_code_block() {
  let text = "Text";
  let res: String = html! { <div>{ text }</div> };
  assert_eq!(res, "<div>Text</div>");
}

#[test]
fn test_functional_component() {
  #[component]
  fn HelloWorld() {
    html! {
      <div>
        <h1>{"Hello"}</h1>
        <p>{"World"}</p>
      </div>
    }
  }

  let result: String = html! { <HelloWorld/> };
  assert_eq!(
    result,
    "<hello-world><div><h1>Hello</h1><p>World</p></div></hello-world>"
  );
}

#[test]
fn test_functional_component_with_attributes() {
  #[component]
  fn PostContent<'a>(title: &'a str, count: i32) {
    html! {
      <div>
        <h1>{title}</h1>
        <p>"Count:" {count}</p>
      </div>
    }
  }

  let result: String = html! { <PostContent title="Hello" count={5} /> };
  assert_eq!(
    result,
    "<post-content title=\"hello\" count=5><div><h1>Hello</h1><p>Count: 5</p></div></hello-world>"
  );
}
