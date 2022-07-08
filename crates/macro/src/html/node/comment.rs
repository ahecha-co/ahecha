use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};

use super::Node;

#[derive(Debug)]
pub struct HtmlComment {
  pub comment: Box<Node>,
}

impl From<HtmlComment> for Node {
  fn from(comment: HtmlComment) -> Self {
    Node::Comment(comment)
  }
}

impl ToTokens for HtmlComment {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    // let comment = &self.comment;
    // quote!( ahecha::html::Node::Comment(ahecha::html::Children::default().set_node( #comment .into())) )
    //   .to_tokens(tokens);
    // TODO: implement comments
    quote!(ahecha::t("")).to_tokens(tokens);
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
      let comment = input.parse::<Node>()?;
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
