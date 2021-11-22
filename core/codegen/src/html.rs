mod attributes;
mod children;
pub mod node;

#[cfg(feature = "html-string-parser")]
mod parser;
#[cfg(feature = "html-string-parser")]
pub use parser::*;
