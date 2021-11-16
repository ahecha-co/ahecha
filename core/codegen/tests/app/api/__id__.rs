use ahecha_codegen::*;

#[api]
pub fn get(id: u32) -> String {
  format!("{{\"title\": \"Hello get {} route\"}}", id)
}

#[api]
pub fn post(id: u32) -> u32 {
  id
}
