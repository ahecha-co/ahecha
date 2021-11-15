use quote::{quote, ToTokens};

use super::HtmlNode;

#[derive(Debug)]
pub struct HtmlComment {
  pub comment: String,
}

impl From<HtmlComment> for HtmlNode {
  fn from(comment: HtmlComment) -> Self {
    HtmlNode::Comment(comment)
  }
}

impl ToTokens for HtmlComment {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let comment = self.to_string();
    quote!( #comment ).to_tokens(tokens);
  }
}

impl ToString for HtmlComment {
  fn to_string(&self) -> String {
    format!("<!-- {} -->", &self.comment)
  }
}
