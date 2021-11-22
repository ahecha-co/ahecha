use ahecha::view::Render;
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
    "<!doctype html><html><head><title>Document title</title></head><body><header class=\"container\"><div class=\"row\"><div class=\"col-9\"/></div></header></body></html>"
  );
}

#[test]
fn test_use_block_in_attribute_value() {
  let res = html! { <div class={"container"}/> };
  assert_eq!(res.render(), "<div class=\"container\"/>");
}

#[test]
fn test_use_expression_block_in_attribute_value() {
  let res = html! { <div class={ 2 + 2u8 }/> };
  assert_eq!(res.render(), "<div class=\"4\"/>");
}

#[test]
fn test_html_with_expression_block() {
  let res = html! { <div>{ 2 + 2u8 }</div> };
  assert_eq!(res.render(), "<div>4</div>");
}
