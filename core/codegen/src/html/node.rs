mod block;
mod comment;
mod custom_element;
mod doctype;
mod element;
mod fragment;
mod partial;
mod text;

use quote::ToTokens;

pub use block::HtmlBlock;
pub use comment::HtmlComment;
pub use custom_element::HtmlCustomElement;
pub use doctype::HtmlDoctype;
pub use element::HtmlElement;
pub use fragment::HtmlFragment;
pub use partial::HtmlPartial;
use syn::parse::Parse;
pub use text::HtmlText;

use crate::html::{attributes::Attributes, children::Children};

#[derive(Debug)]
pub enum HtmlNode {
  Block(HtmlBlock),
  Comment(HtmlComment),
  CustomElement(HtmlCustomElement),
  Doctype(HtmlDoctype),
  Element(HtmlElement),
  Fragment(HtmlFragment),
  Partial(HtmlPartial),
  Text(HtmlText),
}

impl ToTokens for HtmlNode {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    match self {
      HtmlNode::Block(block) => block.to_tokens(tokens),
      HtmlNode::Comment(comment) => comment.to_tokens(tokens),
      HtmlNode::CustomElement(custom_element) => custom_element.to_tokens(tokens),
      HtmlNode::Doctype(doctype) => doctype.to_tokens(tokens),
      HtmlNode::Element(element) => element.to_tokens(tokens),
      HtmlNode::Fragment(fragment) => fragment.to_tokens(tokens),
      HtmlNode::Partial(partial) => partial.to_tokens(tokens),
      HtmlNode::Text(text) => text.to_tokens(tokens),
    }
  }
}

impl ToString for HtmlNode {
  fn to_string(&self) -> String {
    match self {
      HtmlNode::Block(block) => block.to_string(),
      HtmlNode::Comment(comment) => comment.to_string(),
      HtmlNode::CustomElement(custom_element) => custom_element.to_string(),
      HtmlNode::Doctype(doctype) => doctype.to_string(),
      HtmlNode::Element(element) => element.to_string(),
      HtmlNode::Fragment(fragment) => fragment.to_string(),
      HtmlNode::Partial(partial) => partial.to_string(),
      HtmlNode::Text(text) => text.to_string(),
    }
  }
}

impl Parse for HtmlNode {
  fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
    if let Ok(block) = input.parse::<HtmlBlock>() {
      Ok(HtmlNode::Block(block))
    } else if let Ok(comment) = input.parse::<HtmlComment>() {
      Ok(HtmlNode::Comment(comment))
    } else if let Ok(doctype) = input.parse::<HtmlDoctype>() {
      Ok(HtmlNode::Doctype(doctype))
    } else if input.peek(syn::Token![<]) && input.peek2(syn::Ident) {
      input.parse::<syn::Token![<]>()?;
      let name = input.parse::<syn::Ident>()?;
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
        let closing_name = input.parse::<syn::Ident>()?;
        input.parse::<syn::Token![>]>()?;

        if closing_name != name {
          return Err(syn::Error::new(
            closing_name.span(),
            "closing tag does not match opening tag",
          ));
        }

        children
      };

      if name.to_string().chars().next().unwrap().is_uppercase() {
        if name.to_string().ends_with("Partial") {
          Ok(HtmlNode::Partial(HtmlPartial {
            attributes,
            children,
            name,
          }))
        } else {
          Ok(HtmlNode::CustomElement(HtmlCustomElement {
            name,
            attributes,
            children,
          }))
        }
      } else {
        Ok(HtmlNode::Element(HtmlElement {
          name,
          attributes,
          children,
        }))
      }
    } else if let Ok(fragment) = input.parse::<HtmlFragment>() {
      Ok(HtmlNode::Fragment(fragment))
    } else if let Ok(text) = input.parse::<HtmlText>() {
      Ok(HtmlNode::Text(text))
    } else {
      Err(syn::parse::Error::new(input.span(), "expected html node"))
    }
  }
}
