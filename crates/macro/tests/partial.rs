use ahecha::{
  html::{partials::PartialView, Component, Node, RenderString},
  prelude::html,
};
use ahecha_macro::Partial;

#[test]
fn test_partial() {
  #[derive(Partial)]
  struct TestPartial;

  impl Component for TestPartial {
    fn view(&self) -> Node {
      html!(<div>{ "Hello" }</div>)
    }
  }

  let partial = TestPartial;

  assert_eq!("<div>Hello</div>", partial.view().render());
  assert_eq!("TestPartial", TestPartial::id());
}
