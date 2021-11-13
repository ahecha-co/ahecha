#[macro_use]
extern crate ahecha_tuple_list;

pub use html_escaping::escape_html;

mod html;
mod html_escaping;

pub use html::{
  doctype::HtmlDoctype, elements::HtmlElement, fragment::HtmlFragment, render::Render,
};
