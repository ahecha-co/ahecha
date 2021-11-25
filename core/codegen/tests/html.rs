#[cfg(feature = "backend")]
mod backend {
  use ahecha::view::RenderString;
  use ahecha_codegen::*;

  mod ahecha {
    pub use ahecha_view as view;
  }
  #[test]
  fn test_html_tag() {
    let res = html! { <div attr="value">Hello (world)</div> };
    assert_eq!(res.render(), "<div attr=\"value\">Hello (world)</div>");
  }

  #[test]
  fn test_html_tag_with_multiple_attributes() {
    let res = html! { <div attr="value" attr2="value2">Hello</div> };
    assert_eq!(
      res.render(),
      "<div attr=\"value\" attr2=\"value2\">Hello</div>"
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
    "<div class=\"main\"><h1 class=\"heading\">I am a test</h1><p class=\"paragraph\">Lorem ipsum dolor sit amet .</p></div>"
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
    "<!doctype html><html><head><title>Document title</title></head><body><header class=\"container\"><div class=\"row\"><div class=\"col-9\"></div></div></header></body></html>"
  );
  }

  #[test]
  fn test_use_block_in_attribute_value() {
    let res = html! { <div class={"container"}/> };
    assert_eq!(res.render(), "<div class=\"container\"></div>");
  }

  #[test]
  fn test_use_expression_block_in_attribute_value() {
    let res = html! { <div class={ 2 + 2u8 }/> };
    assert_eq!(res.render(), "<div class=\"4\"></div>");
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
    assert_eq!(res.render(), "<div>4</div>");
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
    assert_eq!(res.render(), "<html><area alt=\"text\" class=\"\" coords=\"\" shape=\"\"/><base href=\"https://example.com\" target=\"_blank\"/><br/><col span=\"2\" class=\"batman\"/><embed Required=\"true\" attributes=\"true\"/><hr/><img src=\"images/stickman.gif\" width=\"24\" height=\"39\" alt=\"Stickman\"/><input type=\"text\" name=\"text\" value=\"\"/><link rel=\"stylesheet\" href=\"stylesheet.css\"/><meta name=\"description\" content=\"\"/><param name=\"movie\" value=\"movie.swf\"/><source src=\"movie.ogg\" type=\"video/ogg\"/><track src=\"movie.vtt\" kind=\"subtitles\" srclang=\"en\" label=\"English\"/><wbr/></html>");
  }

  #[test]
  fn test_attribute_data() {
    let res = html! { <div data-tooltip="sum">Data attribute</div> };
    assert_eq!(
      res.render(),
      "<div data-tooltip=\"sum\">Data attribute</div>"
    );
  }

  #[test]
  fn test_attribute_aria() {
    let res = html! { <div aria-label="sum">Aria attribute</div> };
    assert_eq!(res.render(), "<div aria-label=\"sum\">Aria attribute</div>");
  }
}
