use app::api;

use crate::app::SuperUser;

mod app;

#[test]
fn test_generate_route() {
  assert_eq!(api::index::__get_metadata::uri(), "/");
  assert_eq!(api::__id__::__get_metadata::uri(5), "/5");
  assert_eq!(api::__id__::__post_metadata::uri(5), "/5");
}

#[test]
fn test_index_api_request() {
  let response = api::index::get();
  assert_eq!(response, "Hello index api");
}

#[test]
fn test_get_id_api_request() {
  let response = api::__id__::get(5);
  assert_eq!(response, "{\"title\": \"Hello get 5 route\"}");
}

#[test]
fn test_post_id_api_request() {
  let user = SuperUser {
    name: "root".into(),
  };
  let response = api::__id__::post(user, 200);
  assert_eq!(response, 200);
}
