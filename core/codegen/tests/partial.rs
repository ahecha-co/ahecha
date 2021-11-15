use ahecha::view::Render;
use ahecha_codegen::*;

mod ahecha {
  pub use ahecha_view as view;
}

#[test]
fn test_partial() {
  #[partial]
  fn HeadPartial() {
    html! {
      <head><title>I am a partial</title></head>
    }
  }

  let res = html! { <html><HeadPartial /><body></body></html> };
  assert_eq!(
    res.render(),
    "<html><head><title>I am a partial</title></head><body></body></html>"
  );
}

#[test]
fn test_partial_with_block() {
  #[partial]
  fn HeadPartial<'a>(title: &'a str) {
    html! {
      <head><title> { title } </title></head>
    }
  }

  let res = html! { <html><HeadPartial title="I'm a partial" /><body></body></html> };
  assert_eq!(
    res.render(),
    "<html><head><title>I&apos;m a partial</title></head><body></body></html>"
  );
}