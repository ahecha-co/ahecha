use std::any::TypeId;

use ahecha_html::Node;
use axum::extract::ws::WebSocket;

pub trait LiveView {
  type Error: std::error::Error;

  fn id(&self) -> TypeId;

  fn mount(&self) -> Result<Node, Self::Error> {
    self.render()
  }

  fn update(&self) -> Result<(), Self::Error> {
    Ok(())
  }

  fn render(&self) -> Result<Node, Self::Error>;
}

pub struct LiveViewManager {}

impl LiveViewManager {
  pub fn register<V>(&self, live_view: Box<V>) -> Result<Node, V::Error>
  where
    V: LiveView,
  {
    live_view.mount()
  }

  pub fn listen(&self, ws: WebSocket) -> Result<(), ()> {
    Ok(())
  }
}
