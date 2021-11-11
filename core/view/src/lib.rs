#[macro_use]
extern crate ita_tuple_list;

pub use html_escaping::escape_html;

mod backend;
mod html_escaping;

pub use backend::{elements::HtmlElement, render::Render};
