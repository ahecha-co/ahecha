use super::{body::BodyElement, head::HeadElement, render::Render};

pub struct HtmlDocument<T>
where
  T: Render,
{
  head: HeadElement,
  body: BodyElement<T>,
}
