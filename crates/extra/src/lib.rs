use ahecha_html::Node;
pub use ahecha_macro::Page;
pub mod view;

pub use view::Component;
use view::{Layout, Scope};

#[axum::async_trait]
pub trait PageComponent<L>: Send + Sync
where
  L: Layout,
{
  async fn slots(&self) -> L::Slots {
    L::Slots::default()
  }

  async fn view(&self, scope: &mut Scope) -> Result<Node, L::Error>;
}
