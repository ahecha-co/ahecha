use ahecha_codegen::*;

#[api]
pub fn get() -> &'static str {
  "Hello index api"
}
