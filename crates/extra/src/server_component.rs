#[axum::async_trait]
trait ServerComponent: Sized {
  type Error;

  async fn get_props() -> Result<Self, Self::Error>;
}
