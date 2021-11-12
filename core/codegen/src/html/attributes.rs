use quote::{quote, ToTokens};

pub type Attribute = (String, String);

#[derive(Debug, Default)]
pub struct Attributes {
  pub attrs: Vec<Attribute>,
}

impl From<Vec<Attribute>> for Attributes {
  fn from(attrs: Vec<Attribute>) -> Self {
    Self { attrs }
  }
}

impl From<Option<Vec<Attribute>>> for Attributes {
  fn from(attrs: Option<Vec<Attribute>>) -> Self {
    if let Some(attrs) = attrs {
      Self::from(attrs)
    } else {
      Self::default()
    }
  }
}

impl ToTokens for Attributes {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let mut list = quote! { () };

    for (key, value) in self.attrs.iter().rev() {
      list = quote! { ((#key, #value), #list) }
    }

    list.to_tokens(tokens);
  }
}
