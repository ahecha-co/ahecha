use quote::{quote, ToTokens};

use super::HtmlNode;

#[derive(Debug)]
pub struct HtmlComment {
  pub comment: String,
}

impl Into<HtmlNode> for HtmlComment {
  fn into(self) -> HtmlNode {
    return HtmlNode::Comment(self);
  }
}

impl ToTokens for HtmlComment {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let comment = format!("<!-- {} -->", &self.comment);
    quote!( #comment ).to_tokens(tokens);
  }
}
