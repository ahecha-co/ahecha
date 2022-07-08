use axum::Router;

// TODO: enable
// mod v3;

// pub use v3::*;

pub trait Mountable {
  fn mount(router: Router) -> Router;
}
