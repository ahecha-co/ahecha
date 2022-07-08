use quote::{quote, ToTokens};
use syn::parse::Parse;

#[derive(Debug)]
pub struct HtmlText {
  pub text: String,
}

impl ToTokens for HtmlText {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let text = &self.text;
    tokens.extend(quote! {
      ahecha::t(#text)
    });
  }
}

impl ToString for HtmlText {
  fn to_string(&self) -> String {
    self.text.clone()
  }
}

impl Parse for HtmlText {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    if input.peek(syn::token::Brace) || input.peek(syn::Token![<]) {
      return Err(syn::Error::new(input.span(), "Expected text"));
    }

    let text = input.step(|cursor| {
      let mut rest = *cursor;
      let mut text = vec![];
      while let Some((tt, next)) = rest.token_tree() {
        match &tt {
          proc_macro2::TokenTree::Ident(ident) => text.push(ident.to_string()),
          proc_macro2::TokenTree::Punct(punct) => {
            if punct.as_char() == '<' {
              return Ok((text.join(" "), rest));
            }
            text.push(punct.to_string())
          }
          proc_macro2::TokenTree::Literal(lit) => text.push(lit.to_string()),
          proc_macro2::TokenTree::Group(group) => {
            if group.delimiter() == proc_macro2::Delimiter::Brace {
              return Ok((text.join(" "), rest));
            }

            text.push(group.to_string());
          }
        }

        rest = next;
      }

      Ok((text.join(" "), rest))
    })?;

    Ok(HtmlText { text })
  }
}
