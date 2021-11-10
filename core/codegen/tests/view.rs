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
fn test_html_list() {
  let res: String = html! { <ul><li>1</li><li>2</li><li>3</li></ul> };
  assert_eq!(res, "<ul><li>1</li><li>2</li><li>3</li></ul>");
}

//     fn render(&self) -> Html<'a> {
//       html! {
//         <div>
//           <h1>{ self.attributes.title }</h1>
//           <p>{ self.attributes.body }</p>
//         </div>
//       }
//     }
//   }

//   let res: String = html! { <PostElement title="Hello" body="World">"Text"</PostElement> }.into();
//   assert_eq!(
//     res,
//     "<post-element title=\"Hello\" body=\"World\"><h1>Hello</h1><p>World</p></post-element>"
//   );
// }

// #[test]
// fn test_html_with_code_block() {
//   let text = "Text";
//   let res: String = html! { <div>{ text }</div> }.into();
//   assert_eq!(res, "<div>Text</div>");
// }
#[test]
fn test_html_conditional_block() {
  let title = "A title";
  let image = Some("https://cataas.com/cat");
  let res: String = html! {
    <div>
      <h1>{title}</h1>
      { if let Some(image) = image {
        html! {<img src={image} />}
      } else {
        None
      }}
    </div>
  };
  assert_eq!(
    res,
    "<div><h1>A title</h1><img src=\"https://cataas.com/cat\"/></div>"
  );
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
        <p>"Count: " {count}</p>
      </div>
    }
  }

  let result: String = html! { <PostContent title="Hello" count={5} /> };
  assert_eq!(
    result,
    "<post-content title=\"Hello\" count=\"5\"><div><h1>Hello</h1><p>Count: 5</p></div></post-content>"
  );
}

#[test]
fn test_functional_component_with_optional_attributes() {
  #[component]
  fn OptionalContent<'a>(content: Option<&'a str>) {
    html! {
      <div>
        {
          match content {
            Some(content) => html! {<p>{content}</p>},
            _ => None,
          }
        }
      </div>
    }
  }

  let result: String = html! { <OptionalContent content={Some("Something")} /> };
  assert_eq!(
    result,
    "<optional-content content=\"Something\"><div><p>Something</p></div></optional-content>"
  );
}
