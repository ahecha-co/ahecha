use ahecha_html::Node;
pub use ahecha_macro::Page;
// pub mod image;
pub mod page;
// mod record;
pub mod fn_component;
// pub mod router;
// pub mod typed_html;
pub mod view;

#[derive(Copy, Clone)]
pub enum HttpMethod {
  DELETE,
  GET,
  PATCH,
  POST,
  PUT,
}

pub use self::view::Component;
pub use view::{Layout, PageScope};

#[axum::async_trait]
pub trait PageComponent<L>: Send + Sync
where
  L: Layout,
{
  async fn slots(&self) -> L::Slots {
    L::Slots::default()
  }

  async fn view(&self, scope: &mut PageScope) -> Result<Node, L::Error>;
}
