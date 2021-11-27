use ahecha_macro::*;

#[api]
pub fn get() -> &'static str {
  "Hello index api"
}
