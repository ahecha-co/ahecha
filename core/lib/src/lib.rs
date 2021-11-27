pub use ahecha_html as view;
pub use ahecha_macro;

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
