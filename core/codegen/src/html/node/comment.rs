use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};

use super::HtmlNode;

#[derive(Debug)]
pub struct HtmlComment {
  pub comment: Box<HtmlNode>,
}

impl From<HtmlComment> for HtmlNode {
  fn from(comment: HtmlComment) -> Self {
    HtmlNode::Comment(comment)
  }
}

impl ToTokens for HtmlComment {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let comment = &self.comment;
    quote!( #comment ).to_tokens(tokens);
  }
}

impl ToString for HtmlComment {
  fn to_string(&self) -> String {
    format!("<!-- {} -->", &self.comment.to_string())
  }
}

impl Parse for HtmlComment {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    if input.peek(syn::Token![<]) && input.peek2(syn::Token![!]) && input.peek3(syn::Token![-]) {
      input.parse::<syn::Token![<]>()?;
      input.parse::<syn::Token![!]>()?;
      input.parse::<syn::Token![-]>()?;
      input.parse::<syn::Token![-]>()?;
      let comment = input.parse::<HtmlNode>()?;
      input.parse::<syn::Token![-]>()?;
      input.parse::<syn::Token![-]>()?;
      input.parse::<syn::Token![>]>()?;
      Ok(HtmlComment {
        comment: Box::new(comment),
      })
    } else {
      Err(syn::Error::new(input.span(), "expected comment"))
    }
  }
}
