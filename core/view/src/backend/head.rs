pub enum HeadNode {
  Base(HtmlBaseElement),
  Link(HtmlLinkElement),
  Meta(HtmlMetaElement),
  Script(ScriptElement),
  Style(StyleElement),
  Title(HtmlTitleElement),
}

pub struct HeadElement {
  pub children: Vec<HeadNode>,
}

macro_rules! impl_head_element {
  ($($name:ident),*) => {
    $(
      pub struct $name {
        name: String,
        attributes: std::collections::HashMap<String, String>,
      }

      impl crate::backend::render::Render for $name {
        fn render_into<W: std::fmt::Write>(self, writer: &mut W) -> std::fmt::Result {
          write!(writer, "<{}", self.name)?;
          self.attributes.render_into(writer)?;
          write!(writer, "/>")
        }
      }
    )*
  };
}

impl_head_element!(HtmlBaseElement, HtmlLinkElement, HtmlMetaElement);

pub struct HtmlTitleElement {
  name: String,
  attributes: std::collections::HashMap<String, String>,
  title: String,
}

pub struct ScriptElement {
  name: String,
  attributes: std::collections::HashMap<String, String>,
  script: String,
}

pub struct CssRule;

pub struct StyleElement {
  name: String,
  attributes: std::collections::HashMap<String, String>,
  rules: Vec<CssRule>,
}
