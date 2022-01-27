pub use ahecha_html as html;
pub use ahecha_macro::*;
pub use ahecha_record::*;

mod form;
pub mod validate;

pub use form::*;
pub use validate::Validate;

pub mod prelude {
  pub use ahecha_html::*;
  pub use ahecha_macro::*;
}

#[cfg(feature = "frontend")]
mod frontend {
  pub use serde;
  pub use serde_json;
}
#[cfg(feature = "frontend")]
pub use frontend::*;
