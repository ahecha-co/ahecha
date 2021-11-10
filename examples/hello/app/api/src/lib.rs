pub mod posts;

#[cfg(feature = "rocket")]
use rocket::{routes, Route};

#[cfg(feature = "rocket")]
pub fn routes() -> Vec<Route> {
  routes![posts::get, posts::__id__::get]
}
