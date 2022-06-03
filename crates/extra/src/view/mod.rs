use crate::node::{IntoNode, Node};

#[async_trait::async_trait]
pub trait Json<'a>: serde::Deserialize<'a> {
  // TODO: add TypedPath as fn arg
  async fn json<E>(&self) -> Result<(), E>
  where
    E: IntoError;
}

pub trait View {
  fn view(&self) -> Node;
}

pub trait IntoError: IntoJsonError + IntoViewError {}

pub trait IntoJsonError {
  fn into_json_error<'a, E>(self) -> E
  where
    E: serde::Deserialize<'a>;
}

pub trait IntoViewError {
  fn into_view_error(self) -> Node;
}

impl<V> IntoNode for V
where
  V: View,
{
  fn into_node(self) -> Node {
    self.view()
  }
}
