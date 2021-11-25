use app::api;

use crate::app::SuperUser;

mod app;

#[test]
fn test_generate_route() {
  assert_eq!(api::index::get::uri(), "/");
  assert_eq!(api::__id__::get::uri(5), "/5");
  assert_eq!(api::__id__::post::uri(5), "/5");
}

#[test]
fn test_index_api_request() {
  let response = api::index::get::handler();
  assert_eq!(response, "Hello index api");
}

#[test]
fn test_get_id_api_request() {
  let response = api::__id__::get::handler(api::__id__::get::Params { id: 5 });
  assert_eq!(response, "{\"title\": \"Hello get 5 route\"}");
}

#[test]
fn test_post_id_api_request() {
  let user = SuperUser {
    name: "root".into(),
  };
  let response = api::__id__::post::handler(api::__id__::post::Params { id: 200, user });
  assert_eq!(response, 200);
}
