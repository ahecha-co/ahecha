use ita::prelude::*;
use models::Post;
use rocket::serde::json::Json;

pub mod __id__;

pub(crate) const POSTS: [Post; 2] = [
  Post {
    id: 1,
    title: "Hello, world!",
    body: "This is the first post.",
    image: "https://cataas.com/cat",
  },
  Post {
    id: 2,
    title: "Example",
    body: "This is the second post.",
    image: "https://cataas.com/cat",
  },
];

#[route]
pub fn get<'a>() -> Json<Vec<Post<'a>>> {
  Json(POSTS.to_vec())
}
