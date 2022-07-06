use axum_extra::routing::TypedPath;
use http::StatusCode;

use crate::{Children, Element, IntoNode, MaybeRedirect, Node, NodeId};

pub struct LiveView {
  pub id: String,
  pub href: String,
}

impl LiveView {
  pub fn new<I, T>(id: I, href: T) -> Self
  where
    I: ToString,
    T: TypedPath,
  {
    Self {
      id: id.to_string(),
      href: href.to_string(),
    }
  }

  pub fn view(self, inner: Node) -> Node {
    Node::Element(
      Element {
        attributes: crate::Attributes::default()
          .set(Some(("id", self.id.to_string())))
          .set(Some(("href", self.href.to_string()))),
        children: Children::default().set(inner),
        name: "live-view",
      },
      NodeId::new(),
    )
  }
}

impl From<LiveView> for ComponentType {
  fn from(item: LiveView) -> Self {
    ComponentType::LiveView(item)
  }
}

pub enum ComponentType {
  LiveView(LiveView),
  Component,
}

impl ComponentType {
  pub fn view(self, inner: Node) -> Node {
    match self {
      ComponentType::LiveView(live_view) => live_view.view(inner),
      ComponentType::Component => inner,
    }
  }
}

pub trait ComponentError {
  fn message(&self) -> String {
    "".to_string()
  }
}

pub trait Component {
  fn status_code(&self) -> StatusCode {
    StatusCode::OK
  }

  fn ty(&self) -> ComponentType {
    ComponentType::Component
  }

  fn view(self) -> Node;
}

impl<C> IntoNode for C
where
  C: Component,
{
  fn into_node(self) -> Node {
    self.ty().view(self.view())
  }
}

impl<C> From<C> for Node
where
  C: Component,
{
  fn from(item: C) -> Self {
    item.ty().view(item.view())
  }
}

impl<C> Component for Vec<C>
where
  C: Component + Clone,
{
  fn view(self) -> Node {
    Node::Fragment(
      Children {
        children: self.iter().map(|c| c.clone().into_node()).collect(),
      },
      NodeId::new(),
    )
  }
}

impl<C> Component for Option<C>
where
  C: Component,
{
  fn view(self) -> Node {
    match self {
      Some(component) => component.into_node(),
      None => Node::None,
    }
  }
}

impl<S, E> Component for Result<S, E>
where
  S: Component,
  E: Component,
{
  fn view(self) -> Node {
    match self {
      Ok(s) => s.into_node(),
      Err(e) => e.into_node(),
    }
  }
}

impl<C> Component for MaybeRedirect<C>
where
  C: Component,
{
  fn view(self) -> Node {
    match self {
      MaybeRedirect::Redirect(status_code, location) => {
        Node::Redirect(status_code.clone(), location.clone())
      }
      MaybeRedirect::Else(component) => component.into_node(),
    }
  }
}
