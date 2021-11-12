#[macro_use]
extern crate ahecha_tuple_list;

pub use html_escaping::escape_html;

mod backend;
mod html_escaping;

pub use backend::{doctype::HtmlDoctype, elements::HtmlElement, render::Render};
