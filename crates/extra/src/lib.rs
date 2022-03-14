use ahecha_html::Component;

pub trait PageRoute: Component {
  fn mount() -> axum::Router;
}
