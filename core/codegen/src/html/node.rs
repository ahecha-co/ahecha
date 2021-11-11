mod block;
mod component;
mod element;
mod text;

pub use block::HtmlBlock;
pub use component::HtmlComponent;
pub use element::HtmlElement;
use quote::{quote, ToTokens};
pub use text::HtmlText;

#[derive(Debug)]
pub enum HtmlNode {
  Block(HtmlBlock),
  Component(HtmlComponent),
  Element(HtmlElement),
  None,
  Text(HtmlText),
}

impl ToTokens for HtmlNode {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    match self {
      HtmlNode::Block(block) => block.to_tokens(tokens),
      HtmlNode::Component(component) => component.to_tokens(tokens),
      HtmlNode::Element(element) => element.to_tokens(tokens),
      HtmlNode::None => {
        let none = quote! { None };
        none.to_tokens(tokens);
      }
      HtmlNode::Text(text) => text.to_tokens(tokens),
    }
  }
}
