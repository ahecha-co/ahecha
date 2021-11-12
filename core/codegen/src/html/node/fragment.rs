use quote::{quote, ToTokens};

use crate::html::children::Children;

use super::HtmlNode;

#[derive(Debug)]
pub struct HtmlFragment {
  pub children: Children,
}

impl Into<HtmlNode> for HtmlFragment {
  fn into(self) -> HtmlNode {
    HtmlNode::Fragment(self)
  }
}

impl ToTokens for HtmlFragment {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let children = &self.children;
    let element = quote!(
      ahecha::view::HtmlFragment {
        children: #children,
      }
    );
    element.to_tokens(tokens);
  }
}

impl ToString for HtmlFragment {
  fn to_string(&self) -> String {
    "<>...</>".to_string()
  }
}
