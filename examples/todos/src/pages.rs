use dioxus::{events::KeyCode, prelude::*};
use uuid::Uuid;

use crate::api::{CreateTodo, Todo, UpdateTodo};

#[derive(PartialEq, Eq)]
pub enum FilterState {
  All,
  Active,
  Completed,
}

#[::ahecha::page("/")]
#[allow(non_snake_case)]
pub fn Index(cx: Scope) -> Element {
  let filter = use_state(&cx, || FilterState::All);
  let draft = use_state(&cx, || "".to_string());
  let touch = use_state(&cx, || None);

  let todos = use_future(&cx, &touch.clone(), |_| async move {
    reqwest::Client::new()
      .get("http://localhost:3000/api/todos")
      .send()
      .await
      .unwrap()
      .json::<Vec<Todo>>()
      .await
      .unwrap()
  })
  .value()?;

  // Filter the todos based on the filter state
  let mut filtered_todos = todos
    .iter()
    .filter(|item| match **filter {
      FilterState::All => true,
      FilterState::Active => !item.completed,
      FilterState::Completed => item.completed,
    })
    .map(|f| f.id)
    .collect::<Vec<_>>();
  filtered_todos.sort_unstable();

  let show_clear_completed = todos.iter().any(|todo| todo.completed);
  let items_left = filtered_todos.len();
  let item_text = match items_left {
    1 => "item",
    _ => "items",
  };

  cx.render(rsx!{
        section { class: "todoapp",
            div {
                header { class: "header",
                    h1 {"todos"}
                    input {
                        class: "new-todo",
                        placeholder: "What needs to be done?",
                        value: "{draft}",
                        autofocus: "true",
                        oninput: move |evt| draft.set(evt.value.clone()),
                        onkeydown: {
                          let draft = draft.clone();
                          let touch = touch.clone();

                          move |evt| {
                            let draft = draft.clone();
                            let touch = touch.clone();

                            if evt.key_code == KeyCode::Enter && !draft.is_empty() {
                              cx.spawn(async move {
                                let todo = reqwest::Client::new().post("http://localhost:3000/api/todos")
                                  .json(&CreateTodo {
                                    text: draft.to_string(),
                                  })
                                  .send()
                                  .await
                                  .unwrap()
                                  .json::<Todo>()
                                  .await
                                  .unwrap();

                                touch.set(Some(todo.id));
                                draft.set("".to_string());
                              });
                            }
                          }
                        }
                    }
                }
                ul { class: "todo-list",
                    filtered_todos.iter().map(|id| rsx!(TodoEntry { key: "{id}", id: *id, todos: todos.to_vec() }))
                }
                (!todos.is_empty()).then(|| rsx!(
                    footer { class: "footer",
                        span { class: "todo-count",
                            strong {"{items_left} "}
                            span {"{item_text} left"}
                        }
                        ul { class: "filters",
                            li { class: "All", a { onclick: move |_| filter.set(FilterState::All), "All" }}
                            li { class: "Active", a { onclick: move |_| filter.set(FilterState::Active), "Active" }}
                            li { class: "Completed", a { onclick: move |_| filter.set(FilterState::Completed), "Completed" }}
                        }
                        show_clear_completed.then(|| rsx!(
                            button {
                                class: "clear-completed",
                                onclick: {
                                  let touch = touch.clone();
                                  let todos = todos.clone();

                                  move |_| {
                                    let touch = touch.clone();
                                    let todos = todos.clone();

                                    cx.spawn(async move {
                                      for todo in todos.iter().filter(|f| f.completed).collect::<Vec<_>>() {
                                        reqwest::Client::new().delete(format!("http://localhost:3000/api/todos/{}", &todo.id))
                                          .send()
                                          .await
                                          .unwrap();
                                      }

                                      touch.set(None);
                                    });
                                  }
                                },
                                "Clear completed"
                            }
                        ))
                    }
                ))
            }
        }
        footer { class: "info",
            p {"Double-click to edit a todo"}
            p { "Created by ", a {  href: "http://github.com/jkelleyrtp/", "jkelleyrtp" }}
            p { "Part of ", a { href: "http://todomvc.com", "TodoMVC" }}
        }
    })
}

#[derive(Props, PartialEq)]
pub struct TodoEntryProps {
  todos: Vec<Todo>,
  id: Uuid,
}

#[allow(non_snake_case)]
pub fn TodoEntry(cx: Scope<TodoEntryProps>) -> Element {
  let is_editing = use_state(&cx, || false);

  let todos = &cx.props.todos;
  let todo = &todos.iter().find(|f| f.id == cx.props.id)?;
  let completed = if todo.completed { "completed" } else { "" };
  let editing = if **is_editing { "editing" } else { "" };
  let id = cx.props.id;

  let update = |id: Uuid, text: Option<String>, completed: Option<bool>| async move {
    reqwest::Client::new()
      .patch(format!("http://localhost:3000/api/todos/{}", &id))
      .json(&UpdateTodo { completed, text })
      .send()
      .await
      .unwrap();
  };

  cx.render(rsx! {
      li {
          class: "{completed} {editing}",
          div { class: "view",
              input {
                class: "toggle",
                r#type: "checkbox",
                id: "cbg-{todo.id}",
                checked: "{todo.completed}",
                oninput: {
                  move |evt| {
                    let value = evt.value.parse().ok();
                    cx.spawn(async move {
                      update(id, None, value).await;
                    });
                  }
                }
              }

              label {
                  r#for: "cbg-{todo.id}",
                  onclick: move |_| is_editing.set(true),
                  prevent_default: "onclick",
                  "{todo.text}"
              }
          }
          is_editing.then(|| rsx!{
              input {
                  class: "edit",
                  value: "{todo.text}",
                  oninput: {
                    move |evt| {
                      let value = evt.value.parse().ok();
                      cx.spawn(async move {
                        update(id, None, value).await;
                      });
                    }
                  },
                  autofocus: "true",
                  onfocusout: move |_| is_editing.set(false),
                  onkeydown: move |evt| {
                      match evt.key_code {
                          KeyCode::Enter | KeyCode::Escape | KeyCode::Tab => is_editing.set(false),
                          _ => {}
                      }
                  },
              }
          })
      }
  })
}
