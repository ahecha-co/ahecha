use quote::{quote, ToTokens};

#[derive(Debug, PartialEq)]
pub enum AttributeValue {
  Block(String),
  None,
  String(String),
}

impl ToTokens for AttributeValue {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    match self {
      AttributeValue::Block(block) => {
        let block: proc_macro2::TokenStream = block.parse().unwrap();
        quote! {
          #block
        }
        .to_tokens(tokens);
      }
      AttributeValue::None => {}
      AttributeValue::String(s) => quote!(#s).to_tokens(tokens),
    }
  }
}
#[derive(Debug, PartialEq)]
pub struct Attribute {
  pub key: String,
  pub value: AttributeValue,
}

impl Default for Attribute {
  fn default() -> Self {
    Self {
      key: String::new(),
      value: AttributeValue::None,
    }
  }
}

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

    for Attribute { key, value } in self.attrs.iter().rev() {
      list = quote! { ((#key, #value), #list) }
    }

    list.to_tokens(tokens);
  }
}
