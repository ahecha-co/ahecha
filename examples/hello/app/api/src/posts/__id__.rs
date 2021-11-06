use crate::posts::POSTS;
use etagere::route;
use models::Post;
use rocket::serde::json::Json;

#[route]
pub fn get<'a>(id: usize) -> Option<Json<Post<'a>>> {
  if let Some(post) = POSTS.iter().find(|p| p.id == id) {
    Some(Json(post.clone()))
  } else {
    None
  }
}
