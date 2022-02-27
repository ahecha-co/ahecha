pub use html_escaping::escape_html;

mod html;
mod html_escaping;
mod integrations;
mod render;

pub use html::*;
pub use render::RenderString;
