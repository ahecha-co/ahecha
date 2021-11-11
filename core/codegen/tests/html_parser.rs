use ita_codegen::*;

mod ita {
  pub use ita_view as view;
}

#[test]
fn test_html_tag() {
  let res: String = html_parser! { <div attr="value">Hello</div> };
  assert_eq!(res, "<div attr=\"value\">Hello</div>");
}