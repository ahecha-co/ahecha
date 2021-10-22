/// Scans the current crate searching all route/page attribute macros and expose them in a constant
pub fn generate_routes(_path: &'static str) -> &'static str {
  "use rocket::{routes, Route};

pub const ROUTES: Vec<Route> = routes![];
"
}
