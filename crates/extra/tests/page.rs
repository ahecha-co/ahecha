use ahecha_extra::view::{Component, Layout, View};
use ahecha_html::Node;
use ahecha_macro::{html, Page};
use axum::extract::{FromRequest, RequestParts};

// #[derive(Page)]
// #[route("/simple_route")]
// struct SimplePage {}

// #[axum::async_trait]
// impl Component for SimplePage {
//   async fn view(&self) -> Node {
//     html!(<span>Hello component</span>)
//   }
// }

// #[derive(Page)]
// #[route("/with_params/:id")]
// struct WithParamsPage {
//   id: String,
// }

// #[axum::async_trait]
// impl Component for WithParamsPage {
//   async fn view(&self) -> Node {
//     html!(<span>Hello component</span>)
//   }
// }

pub struct TestLayout;

impl Layout for TestLayout {
  type Props = ();
  type Slots = ();

  fn render<'a, P>(&self, scope: ahecha_extra::view::Scope<P, Self::Slots>, body: Node) -> Node {
    todo!()
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
struct PartialPage {
  view: View<TestLayout>,
}

#[axum::async_trait]
impl Component for PartialPage {
  async fn view(&self) -> Node {
    html!(<div>Hello async component</div>)
  }
}
