#[cfg(feature = "backend")]
mod backend {
  use ahecha::{macros::html, render_to_string};
  use pretty_assertions::assert_eq;

  fn example() {
    ahecha::sycamore::render(|cx| {
      let data = vec!["hello", "world"];
      ahecha::sycamore::view! { cx,
        div {
          (data.iter().map(|i| html!(<li>{ i }</li>)).collect::<Vec<_>>())
        }
      }
    });
  }

  #[test]
  fn test_html_tag() {
    let res = html! { <div attr="value">Hello (world)</div> };
    assert_eq!(
      render_to_string(|cx| res.view(cx)),
      r#"<div attr="value">Hello (world)</div>"#
    );
  }

  #[test]
  fn test_html_tag_with_multiple_attributes() {
    let res = html! { <div attr="value" attr2="value2">Hello</div> };
    assert_eq!(
      render_to_string(|cx| res.view(cx)),
      r#"<div attr="value" attr2="value2">Hello</div>"#
    );
  }

  #[test]
  fn test_html_tag_nested() {
    let res = html! {
      <div class="main">
        <h1 class="heading">I am a test</h1>
        <p class="paragraph">Lorem ipsum dolor sit amet.</p>
      </div>
    };
    assert_eq!(
      render_to_string(|cx| res.view(cx)),
      r#"<div class="main"><h1 class="heading">I am a test</h1><p class="paragraph">Lorem ipsum dolor sit amet .</p></div>"#
    );
  }

  #[test]
  fn test_html_with_doctype() {
    let res = html! {
      // <!doctype html>
      <html>
        <head>
          <title>Document title</title>
        </head>
        <body>
          <header class="container">
            <div class="row">
              <div class="col-9"></div>
            </div>
          </header>
        </body>
      </html>
    };
    assert_eq!(
      render_to_string(|cx| res.view(cx)),
      r#"<!doctype html><html><head><title>Document title</title></head><body><header class="container"><div class="row"><div class="col-9"></div></div></header></body></html>"#
    );
  }

  #[test]
  fn test_html_with_stylesheet() {
    let res = html! {
      // <!doctype html>
      <html>
        <head>
          <link rel="stylesheet" href="/stylesheet.css"/>
        </head>
        <body></body>
      </html>
    };
    assert_eq!(
      render_to_string(|cx| res.view(cx)),
      r#"<!doctype html><html><head><link rel="stylesheet" href="/stylesheet.css"/></head><body></body></html>"#
    );
  }

  #[test]
  fn test_use_block_in_attribute_value() {
    let res = html! { <div class={"container"}/> };
    assert_eq!(
      render_to_string(|cx| res.view(cx)),
      r#"<div class="container"></div>"#
    );
  }

  #[test]
  fn test_use_expression_block_in_attribute_value() {
    let res = html! { <div class={format!("{}",  2 + 2u8)}/> };
    assert_eq!(
      render_to_string(|cx| res.view(cx)),
      r#"<div class="4"></div>"#
    );
  }

  #[test]
  fn test_html_with_expression_block() {
    let res = html! { <div>{(2 + 2u8)}</div> };
    assert_eq!(render_to_string(|cx| res.view(cx)), "<div>4</div>");
  }

  #[test]
  fn test_children_iter() {
    let data = vec!["hello", "world"];
    let res = render_to_string(|cx| {
      html! { <ul>{ data.iter().map(|i| html!(<li>{ i }</li>).view(cx)).collect::<Vec<_>>() }</ul> }
        .view(cx)
    });
    assert_eq!(res, "<ul><li>hello</li><li>world</li></ul>");
  }

  // TODO: implement fragments correctly
  // #[test]
  // fn test_fragment() {
  //   let res = html! { <div><><>{ 2 + 2u8 }</></></div> };
  //   assert_eq!(render_to_string(|cx| res.view(cx)), "<div>4</div>");
  // }

  #[test]
  fn test_comments() {
    let res = html! { <div><!--{ 2 + 2u8 }--></div> };
    assert_eq!(render_to_string(|cx| res.view(cx)), "<div><!--4--></div>");
  }

  #[test]
  fn test_self_closing_tags() {
    let res = html! {
      <html>
        <area alt="text" class="" coords="" shape="">
        <base href="https://example.com" target="_blank">
        <br>
        <col span="2" class="batman">
        <embed required attributes>
        <hr>
        <img src="images/stickman.gif" width="24" height="39" alt="Stickman">
        <input type="text" name="text" value="">
        <link rel="stylesheet" href="stylesheet.css">
        <meta name="description" content="">
        <param name="movie" value="movie.swf">
        <source src="movie.ogg" type="video/ogg">
        <track src="movie.vtt" kind="subtitles" srclang="en" label="English">
        <wbr>
      </html>
    };
    assert_eq!(
      render_to_string(|cx| res.view(cx)),
      r#"<html><area alt="text" class coords shape/><base href="https://example.com" target="_blank"/><br/><col span="2" class="batman"/><embed Required attributes/><hr/><img src="images/stickman.gif" width="24" height="39" alt="Stickman"/><input type="text" name="text" value/><link rel="stylesheet" href="stylesheet.css"/><meta name="description" content/><param name="movie" value="movie.swf"/><source src="movie.ogg" type="video/ogg"/><track src="movie.vtt" kind="subtitles" srclang="en" label="English"/><wbr/></html>"#
    );
  }

  #[test]
  fn test_attribute_data() {
    let res = html! { <div data-tooltip="sum">Data attribute</div> };
    assert_eq!(
      render_to_string(|cx| res.view(cx)),
      r#"<div data-tooltip="sum">Data attribute</div>"#
    );
  }

  #[test]
  fn test_attribute_aria() {
    let res = html! { <div aria-label="sum">Aria attribute</div> };
    assert_eq!(
      render_to_string(|cx| res.view(cx)),
      r#"<div aria-label="sum">Aria attribute</div>"#
    );
  }

  #[test]
  fn test_parse_custom_element_tag_name() {
    let res = html! { <word-count>Aria attribute</word-count> };
    assert_eq!(
      render_to_string(|cx| res.view(cx)),
      "<word-count>Aria attribute</word-count>"
    );
  }

  // TODO: implement optional params
  // #[test]
  // #[allow(unused_braces)]
  // fn test_optional_tag_attribute() {
  //   let class: Option<&'static str> = None;
  //   let res = html! { <div class={class}>Aria attribute</div> };
  //   assert_eq!(
  //     render_to_string(|cx| res.view(cx)),
  //     r#"<div class="test">Aria attribute</div>"#
  //   );
  // }

  #[test]
  fn test_support_string_reference_for_text_nodes() {
    let text = "Hello World".to_string();
    let res = html! { <div>{&text}</div> };
    assert_eq!(
      render_to_string(|cx| res.view(cx)),
      r#"<div>Hello World</div>"#
    );
  }

  // #[test]
  // fn test_support_everything_that_impl_display_trait_for_text_nodes() {
  //   struct Hello;

  //   impl std::fmt::Display for Hello {
  //     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
  //       write!(f, "Hello ")
  //     }
  //   }

  //   struct World;

  //   impl std::fmt::Display for World {
  //     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
  //       write!(f, "World")
  //     }
  //   }

  //   let res = html! { <div>{Hello} {&World}</div> };
  //   assert_eq!(render_to_string(|cx| res.view(cx)), r#"<div>Hello World</div>"#);
  // }
}
