use ahecha_codegen::*;

use crate::app::SuperUser;

#[api]
pub fn get(id: u32) -> String {
  format!("{{\"title\": \"Hello get {} route\"}}", id)
}

#[api]
pub fn post(user: SuperUser, id: u32) -> u32 {
  id
}
