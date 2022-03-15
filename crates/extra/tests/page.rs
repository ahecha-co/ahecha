use ahecha_extra::AsyncComponent;
use ahecha_html::{partials::PartialBuilder, Component, Node};
use ahecha_macro::{html, Page};
use async_trait::async_trait;

#[derive(Page)]
#[route("/simple_route")]
struct SimplePage {}

impl Component for SimplePage {
  fn view(&self) -> Node {
    html!(<span>Hello component</span>)
  }
}

#[derive(Page)]
#[route("/partial_page")]
struct PartialPage {
  partial: PartialBuilder,
}

#[async_trait]
impl AsyncComponent for PartialPage {
  async fn view(&mut self) -> Node {
    html!(<div>Hello async component</div>)
  }
}
