// use etagere::route;

// Each part of the module path/name that contains `__{str}__` is treated as a route param, so you
// use them in your route and define the accepted parameter type.

// Example of a mutation query it will be mapped to POST /api/query.
// POST, PUT, PATCH, DELETE are considered mutation queries.
// #[route]
// pub fn post(id: usize) -> &'static str {
//   ""
// }
