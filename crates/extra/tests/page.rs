use ahecha_extra::{
  view::{Layout, Scope},
  PageComponent,
};
use ahecha_html::Node;
use ahecha_macro::{html, Page};
use axum::extract::{FromRequest, RequestParts};

#[derive(Page)]
#[route("/simple_route")]
#[layout(TestLayout)]
struct SimplePage {}

#[axum::async_trait]
impl PageComponent<TestLayout> for SimplePage {
  async fn view(&self, _: &mut Scope) -> Result<Node, <TestLayout as Layout>::Error> {
    Ok(html!(<span>Hello component</span>))
  }
}

#[derive(Page)]
#[route("/with_params/:id")]
#[layout(TestLayout)]
struct WithParamsPage {
  id: String,
}

#[axum::async_trait]
impl PageComponent<TestLayout> for WithParamsPage {
  async fn view(&self, _: &mut Scope) -> Result<Node, <TestLayout as Layout>::Error> {
    Ok(html!(<span>Hello component {&self.id}</span>))
  }
}

pub struct TestLayout;

impl Layout for TestLayout {
  type Error = ();
  type Slots = ();

  fn can_render_errors(&self) -> bool {
    false
  }

  fn render(&self, _: Self::Slots, body: Node) -> Node {
    body
  }
}

#[axum::async_trait]
impl<B> FromRequest<B> for TestLayout
where
  B: Send,
{
  type Rejection = http::StatusCode;

  async fn from_request(_: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
    Ok(Self)
  }
}

#[derive(Page)]
#[route("/partial_page")]
#[layout(TestLayout)]
struct PartialPage;

#[axum::async_trait]
impl PageComponent<TestLayout> for PartialPage {
  async fn view(&self, _: &mut Scope) -> Result<Node, <TestLayout as Layout>::Error> {
    Ok(html!(<div>Hello async component</div>))
  }
}
