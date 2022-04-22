/*
The idea is to support the html5 spec + web components and some other specific things from the crate.

TODO:
 - Support most of the html5 spec.
 - Support web components.
 - Support partials (the dom rendered when requested a partial page, like from live views).
 - Support routing and nested routing.
 - Support hydration
*/
use std::fmt::Display;

pub enum Doctype {
  Html,
}

impl Display for Doctype {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "<!DOCTYPE {}>",
      match self {
        Doctype::Html => "html",
      }
    )
  }
}

impl Default for Doctype {
  fn default() -> Self {
    Doctype::Html
  }
}

pub struct Document {
  doctype: Doctype,
  html: Html,
}

impl Display for Document {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}{}", self.doctype, self.html)
  }
}

pub struct Head {
  // children: MetadataContent,
}

impl Display for Head {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "")
  }
}

pub enum FlowContent {
  // ...
  Component,
  Partial,
}

pub struct Body {
  children: FlowContent,
}

impl Display for Body {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "")
  }
}

pub struct HtmlAttrs {}

impl Display for HtmlAttrs {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "")
  }
}

pub struct Html {
  attrs: HtmlAttrs,
  children: (Head, Body),
}

impl Display for Html {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "<html{}>{}{}</html>",
      self.attrs, self.children.0, self.children.1
    )
  }
}
