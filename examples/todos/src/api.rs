#[cfg(not(target_arch = "wasm32"))]
use axum::{
  extract::{Path, Query},
  response::IntoResponse,
  Extension, Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(not(target_arch = "wasm32"))]
use crate::Db;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct Todo {
  pub id: Uuid,
  pub text: String,
  pub completed: bool,
}

#[derive(Serialize, Deserialize)]
pub enum JsonResponse {
  Created,
  Deleted,
}

#[derive(Serialize, Deserialize)]
pub enum JsonError {
  NotFound,
}

#[cfg(not(target_arch = "wasm32"))]
impl IntoResponse for JsonError {
  fn into_response(self) -> axum::response::Response {
    Json(self).into_response()
  }
}

// The query parameters for todos index
#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
  pub offset: Option<usize>,
  pub limit: Option<usize>,
}

#[::ahecha::route(GET, "/todos")]
pub async fn index(
  pagination: Option<Query<Pagination>>,
  Extension(db): Extension<Db>,
) -> Json<Vec<Todo>> {
  let todos = db.read().unwrap();

  let Query(pagination) = pagination.unwrap_or_default();

  let todos = todos
    .values()
    .skip(pagination.offset.unwrap_or(0))
    .take(pagination.limit.unwrap_or(usize::MAX))
    .cloned()
    .collect::<Vec<_>>();

  Json(todos)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTodo {
  pub text: String,
}

#[::ahecha::route(POST, "/todos")]
pub async fn create(Extension(db): Extension<Db>, Json(input): Json<CreateTodo>) -> Json<Todo> {
  let todo = Todo {
    id: Uuid::new_v4(),
    text: input.text,
    completed: false,
  };

  db.write().unwrap().insert(todo.id, todo.clone());

  Json(todo)
}

#[::ahecha::route(DELETE, "/todos/:id")]
pub async fn delete(
  Path(id): Path<Uuid>,
  Extension(db): Extension<Db>,
) -> Result<Json<JsonResponse>, JsonError> {
  if db.write().unwrap().remove(&id).is_some() {
    Ok(Json(JsonResponse::Deleted))
  } else {
    Err(JsonError::NotFound)
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateTodo {
  pub text: Option<String>,
  pub completed: Option<bool>,
}

#[::ahecha::route(PATCH, "/todos/:id")]
pub async fn update(
  Path(id): Path<Uuid>,
  Extension(db): Extension<Db>,
  Json(input): Json<UpdateTodo>,
) -> Result<Json<Todo>, JsonError> {
  let mut todo = db
    .read()
    .unwrap()
    .get(&id)
    .cloned()
    .ok_or(JsonError::NotFound)?;

  if let Some(text) = input.text {
    todo.text = text;
  }

  if let Some(completed) = input.completed {
    todo.completed = completed;
  }

  db.write().unwrap().insert(todo.id, todo.clone());

  Ok(Json(todo))
}
