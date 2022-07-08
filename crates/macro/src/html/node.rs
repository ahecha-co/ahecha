mod block;
mod comment;
mod doctype;
mod element;
mod fragment;
mod text;

pub use block::HtmlBlock;
pub use comment::HtmlComment;
pub use doctype::HtmlDoctype;
pub use element::HtmlElement;
pub use fragment::HtmlFragment;
use quote::ToTokens;
use syn::parse::Parse;
pub use text::HtmlText;

use super::tag_name::TagName;
use crate::html::{attributes::Attributes, children::Children};

#[derive(Debug)]
pub enum Node {
  Block(HtmlBlock),
  Comment(HtmlComment),
  Doctype(HtmlDoctype),
  Element(HtmlElement),
  Fragment(HtmlFragment),
  Text(HtmlText),
}

impl ToTokens for Node {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    match self {
      Node::Block(block) => block.to_tokens(tokens),
      Node::Comment(comment) => comment.to_tokens(tokens),
      Node::Doctype(doctype) => doctype.to_tokens(tokens),
      Node::Element(element) => element.to_tokens(tokens),
      Node::Fragment(fragment) => fragment.to_tokens(tokens),
      Node::Text(text) => text.to_tokens(tokens),
    }
  }
}

impl ToString for Node {
  fn to_string(&self) -> String {
    match self {
      Node::Block(block) => block.to_string(),
      Node::Comment(comment) => comment.to_string(),
      Node::Doctype(doctype) => doctype.to_string(),
      Node::Element(element) => element.to_string(),
      Node::Fragment(fragment) => fragment.to_string(),
      Node::Text(text) => text.to_string(),
    }
  }
}

impl Parse for Node {
  fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
    if let Ok(block) = input.parse::<HtmlBlock>() {
      Ok(Node::Block(block))
    } else if let Ok(comment) = input.parse::<HtmlComment>() {
      Ok(Node::Comment(comment))
    } else if let Ok(doctype) = input.parse::<HtmlDoctype>() {
      Ok(Node::Doctype(doctype))
    } else if input.peek(syn::Token![<]) && input.peek2(syn::Ident) {
      input.parse::<syn::Token![<]>()?;
      let name = input.parse::<TagName>()?;
      let attributes = input.parse::<Attributes>()?;

      let self_closing_tags = [
        "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param",
        "source", "track", "wbr",
      ];
      let self_closing = input.parse::<syn::Token![/]>().is_ok()
        || self_closing_tags.contains(&name.to_string().as_str());

      input.parse::<syn::Token![>]>()?;

      let children = if self_closing {
        Children::default()
      } else {
        let children = input.parse::<Children>()?;
        input.parse::<syn::Token![<]>()?;
        input.parse::<syn::Token![/]>()?;
        let closing_name = input.parse::<TagName>()?;
        input.parse::<syn::Token![>]>()?;

        if closing_name != name {
          return Err(syn::Error::new(
            closing_name.span(),
            "closing tag does not match opening tag",
          ));
        }

        children
      };

      Ok(Node::Element(HtmlElement {
        name,
        attributes,
        children,
      }))
    } else if let Ok(fragment) = input.parse::<HtmlFragment>() {
      Ok(Node::Fragment(fragment))
    } else if let Ok(text) = input.parse::<HtmlText>() {
      Ok(Node::Text(text))
    } else {
      Err(syn::parse::Error::new(input.span(), "expected html node"))
    }
  }
}
