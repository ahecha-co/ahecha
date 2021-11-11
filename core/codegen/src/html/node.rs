mod block;
mod comment;
mod component;
mod doctype;
mod element;
mod text;

pub use block::HtmlBlock;
pub use comment::HtmlComment;
pub use component::HtmlComponent;
pub use doctype::HtmlDoctype;
pub use element::HtmlElement;
use quote::ToTokens;
pub use text::HtmlText;

#[derive(Debug)]
pub enum HtmlNode {
  Block(HtmlBlock),
  Comment(HtmlComment),
  Component(HtmlComponent),
  Doctype(HtmlDoctype),
  Element(HtmlElement),
  Text(HtmlText),
}

impl ToTokens for HtmlNode {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    match self {
      HtmlNode::Block(block) => block.to_tokens(tokens),
      HtmlNode::Comment(comment) => comment.to_tokens(tokens),
      HtmlNode::Component(component) => component.to_tokens(tokens),
      HtmlNode::Doctype(doctype) => doctype.to_tokens(tokens),
      HtmlNode::Element(element) => element.to_tokens(tokens),
      HtmlNode::Text(text) => text.to_tokens(tokens),
    }
  }
}
