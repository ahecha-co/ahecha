use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};

use crate::html::{attributes::Attributes, children::Children};

use super::{HtmlCustomElement, HtmlNode, HtmlPartial};

#[derive(Debug)]
pub struct HtmlElement {
  pub attributes: Attributes,
  pub children: Children,
  pub name: syn::Ident,
}

impl From<HtmlElement> for HtmlNode {
  fn from(element: HtmlElement) -> Self {
    if element
      .name
      .to_string()
      .chars()
      .next()
      .unwrap_or_default()
      .is_uppercase()
    {
      if element.name.to_string().ends_with("Partial") {
        HtmlNode::Partial(HtmlPartial {
          attributes: element.attributes,
          children: element.children,
          name: element.name,
        })
      } else {
        HtmlNode::CustomElement(HtmlCustomElement {
          attributes: element.attributes,
          children: element.children,
          name: element.name,
        })
      }
    } else {
      HtmlNode::Element(element)
    }
  }
}

impl ToTokens for HtmlElement {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let attributes = &self.attributes;
    let children = &self.children;
    let name = &self.name;

    let element = quote!(
      ahecha::view::HtmlElement {
        attributes: #attributes,
        children: #children,
        name: stringify!(#name),
      }
    );
    element.to_tokens(tokens);
  }
}

impl ToString for HtmlElement {
  fn to_string(&self) -> String {
    format!("<{}>...</{}>", self.name, self.name)
  }
}

impl Parse for HtmlElement {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    if input.peek(syn::Token![<]) && input.peek2(syn::Token![/]) {
      return Err(input.error("unexpected closing tag"));
    }

    dbg!(input.to_string());
    input.parse::<syn::Token![<]>()?;
    let name = if input.peek(syn::Token![>]) {
      syn::Ident::new("", proc_macro2::Span::call_site())
    } else {
      input.parse()?
    };

    let attributes = input.parse()?;

    // Self closing tag
    let self_closing = input.parse::<syn::Token![/]>().is_ok();

    input.parse::<syn::Token![>]>()?;

    let children = if self_closing {
      Children::default()
    } else {
      let children = input.parse::<Children>()?;
      input.parse::<syn::Token![<]>()?;
      input.parse::<syn::Token![/]>()?;
      let closing_name = input.parse::<syn::Ident>()?;

      if closing_name != name {
        return Err(syn::Error::new(
          proc_macro2::Span::call_site(),
          "Closing tag name does not match opening tag name",
        ));
      }

      input.parse::<syn::Token![>]>()?;

      children
    };

    // dbg!(
    //   "HtmlElement {:?}[{:?}] => input",
    //   &name,
    //   &self_closing,
    //   input
    // );

    Ok(
      HtmlElement {
        attributes,
        children,
        name,
      }
      .into(),
    )
  }
}
