pub use html_escaping::escape_html;

mod component;
mod html;
mod html_escaping;
mod integrations;
#[cfg(feature = "partials")]
pub mod partials;
mod render;

pub use self::{component::*, html::*, render::RenderString};
