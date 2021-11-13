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
pub use text::HtmlText;

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
