pub use html_escaping::escape_html;
use http::{StatusCode, Uri};

mod component;
mod html;
mod html_escaping;
mod integrations;
// #[cfg(feature = "partials")]
// pub mod partials;
mod render;

pub use self::{component::*, html::*, render::RenderString};

// TODO: improve redirection
pub enum MaybeRedirect<C>
where
  C: Component,
{
  Redirect(StatusCode, Uri),
  Else(C),
}
