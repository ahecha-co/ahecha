pub mod view;

pub use view::Component;

pub trait PageRoute: Component {
  fn mount() -> axum::Router;
}
