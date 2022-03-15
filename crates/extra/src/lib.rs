pub use ahecha_html::Component;
use ahecha_html::Node;
use axum::async_trait;

pub trait AsyncPageRoute: AsyncComponent {
  fn mount() -> axum::Router;
}

pub trait PageRoute: Component {
  fn mount() -> axum::Router;
}

#[async_trait]
pub trait AsyncComponent {
  async fn view(&mut self) -> Node;
}
