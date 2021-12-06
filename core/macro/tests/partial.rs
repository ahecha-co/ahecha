use ahecha::html::*;
use ahecha_macro::*;

mod ahecha {
  pub use ahecha_html as html;
}

#[test]
fn test_partial() {
  #[partial]
  fn HeadPartial() -> ahecha::html::Node {
    html! {
      <head><title>I am a partial</title></head>
    }
  }

  let res = html! { <html><HeadPartial /><body></body></html> }.render();
  assert_eq!(
    res,
    "<html><head><title>I am a partial</title></head><body></body></html>"
  );
}

#[test]
fn test_partial_with_block() {
  #[partial]
  fn HeadPartial<'a>(title: &'a str) -> ahecha::html::Node {
    html! {
      <head><title> { title } </title></head>
    }
  }

  let res = html! { <html><HeadPartial title="I'm a partial" /><body></body></html> }.render();
  assert_eq!(
    res,
    "<html><head><title>I&apos;m a partial</title></head><body></body></html>"
  );
}
