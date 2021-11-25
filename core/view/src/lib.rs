pub use html_escaping::escape_html;

mod html;
mod html_escaping;
mod render;

pub use html::{doctype::HtmlDoctype, elements::HtmlElement, fragment::HtmlFragment};
pub use render::{RenderNode, RenderString};
