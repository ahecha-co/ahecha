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
    // dbg!(
    //   "\n\n ======== \n\n HtmlNode::parse {:?} \n\n",
    //   input.to_string()
    // );
    if let Ok(block) = input.parse::<HtmlBlock>() {
      Ok(HtmlNode::Block(block))
    // } else if let Ok(comment) = input.parse::<HtmlComment>() {
    //   Ok(HtmlNode::Comment(comment))
    // // } else if let Ok(custom_element) = input.parse::<HtmlCustomElement>() {
    // //   Ok(HtmlNode::CustomElement(custom_element))
    } else if let Ok(doctype) = input.parse::<HtmlDoctype>() {
      Ok(HtmlNode::Doctype(doctype))
    } else if let Ok(element) = input.parse::<HtmlElement>() {
      Ok(HtmlNode::Element(element))
    // } else if let Ok(fragment) = input.parse::<HtmlFragment>() {
    //   Ok(HtmlNode::Fragment(fragment))
    // } else if let Ok(partial) = input.parse::<HtmlPartial>() {
    //   Ok(HtmlNode::Partial(partial))
    } else if let Ok(text) = input.parse::<HtmlText>() {
      Ok(HtmlNode::Text(text))
    } else {
      Err(syn::parse::Error::new(input.span(), "expected html node"))
    }
  }
}
