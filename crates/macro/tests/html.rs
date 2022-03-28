#[cfg(feature = "backend")]
mod backend {
  use ahecha::{
    html::{Node, RenderString},
    macros::html,
  };
  use pretty_assertions::assert_eq;

  #[test]
  fn test_html_tag() {
    let res = html! { <div attr="value">Hello (world)</div> };
    assert_eq!(res.render(), r#"<div attr="value">Hello (world)</div>"#);
  }

  #[test]
  fn test_html_tag_with_multiple_attributes() {
    let res = html! { <div attr="value" attr2="value2">Hello</div> };
    assert_eq!(
      res.render(),
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
      res.render(),
      r#"<div class="main"><h1 class="heading">I am a test</h1><p class="paragraph">Lorem ipsum dolor sit amet .</p></div>"#
    );
  }

  #[test]
  fn test_html_with_doctype() {
    let res = html! {
      <!doctype html>
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
      res.render(),
      r#"<!doctype html><html><head><title>Document title</title></head><body><header class="container"><div class="row"><div class="col-9"></div></div></header></body></html>"#
    );
  }

  #[test]
  fn test_html_with_stylesheet() {
    let res = html! {
      <!doctype html>
      <html>
        <head>
          <link rel="stylesheet" href="/stylesheet.css"/>
        </head>
        <body></body>
      </html>
    };
    assert_eq!(
      res.render(),
      r#"<!doctype html><html><head><link rel="stylesheet" href="/stylesheet.css"/></head><body></body></html>"#
    );
  }

  #[test]
  fn test_use_block_in_attribute_value() {
    let res = html! { <div class={"container"}/> };
    assert_eq!(res.render(), r#"<div class="container"></div>"#);
  }

  #[test]
  fn test_use_expression_block_in_attribute_value() {
    let res = html! { <div class={ 2 + 2u8 }/> };
    assert_eq!(res.render(), r#"<div class="4"></div>"#);
  }

  #[test]
  fn test_html_with_expression_block() {
    let res = html! { <div>{ 2 + 2u8 }</div> };
    assert_eq!(res.render(), "<div>4</div>");
  }

  #[test]
  fn test_children_iter() {
    let data = vec!["hello", "world"];
    let res = html! { <ul>{ data.iter().map(|i| html!(<li>{ i }</li>)).collect::<Vec<_>>() }</ul> };
    assert_eq!(res.render(), "<ul><li>hello</li><li>world</li></ul>");
  }

  #[test]
  fn test_fragment() {
    let res = html! { <div><><>{ 2 + 2u8 }</></></div> };
    assert_eq!(res.render(), "<div>4</div>");
  }

  #[test]
  fn test_comments() {
    let res = html! { <div><!--{ 2 + 2u8 }--></div> };
    assert_eq!(res.render(), "<div><!--4--></div>");
  }

  #[test]
  fn test_self_closing_tags() {
    let res = html! {
      <html>
        <area alt="text" class="" coords="" shape="">
        <base href="https://example.com" target="_blank">
        <br>
        <col span="2" class="batman">
        <embed Required attributes>
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
      res.render(),
      r#"<html><area alt="text" class coords shape/><base href="https://example.com" target="_blank"/><br/><col span="2" class="batman"/><embed Required attributes/><hr/><img src="images/stickman.gif" width="24" height="39" alt="Stickman"/><input type="text" name="text" value/><link rel="stylesheet" href="stylesheet.css"/><meta name="description" content/><param name="movie" value="movie.swf"/><source src="movie.ogg" type="video/ogg"/><track src="movie.vtt" kind="subtitles" srclang="en" label="English"/><wbr/></html>"#
    );
  }

  #[test]
  fn test_attribute_data() {
    let res = html! { <div data-tooltip="sum">Data attribute</div> };
    assert_eq!(
      res.render(),
      r#"<div data-tooltip="sum">Data attribute</div>"#
    );
  }

  #[test]
  fn test_attribute_aria() {
    let res = html! { <div aria-label="sum">Aria attribute</div> };
    assert_eq!(
      res.render(),
      r#"<div aria-label="sum">Aria attribute</div>"#
    );
  }

  #[test]
  fn test_parse_custom_element_tag_name() {
    let res = html! { <word-count>Aria attribute</word-count> };
    assert_eq!(res.render(), "<word-count>Aria attribute</word-count>");
  }

  #[test]
  #[allow(unused_braces)]
  fn test_optional_tag_attribute() {
    let class = Some(("class", "test"));
    let selected = Option::<(&str, &str)>::None;
    let res = html! { <div {selected} {class}>Aria attribute</div> };
    assert_eq!(res.render(), r#"<div class="test">Aria attribute</div>"#);
  }

  #[test]
  fn test_support_string_reference_for_text_nodes() {
    let text = "Hello World".to_string();
    let res = html! { <div>{&text}</div> };
    assert_eq!(res.render(), r#"<div>Hello World</div>"#);
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
  //   assert_eq!(res.render(), r#"<div>Hello World</div>"#);
  // }

  #[test]
  fn test_live_view() {
    let res = html! { <div><live-view event="test" action="/partial">I am alive</live-view></div> };
    assert_eq!(
      res.render(),
      r#"<div><live-view event="test" action="/partial">I am alive</live-view></div>"#
    );
  }

  #[test]
  fn test_find_live_view() {
    let res: Node = html! { <div><live-view event="test" action="/partial"><span>I am alive</span></live-view></div> };
    let view = res.find_live_view("/partial");
    assert_eq!(view.unwrap().render(), r#"<span>I am alive</span>"#);
  }
}
