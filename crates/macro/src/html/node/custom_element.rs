use quote::ToTokens;

use crate::html::{attributes::Attributes, children::Children, tag_name::TagName};

#[derive(Debug)]
pub struct HtmlCustomElement {
  pub attributes: Attributes,
  pub children: Children,
  pub name: TagName,
}

impl ToTokens for HtmlCustomElement {
  fn to_tokens(&self, _tokens: &mut proc_macro2::TokenStream) {
    //   let ident = &self.name;
    //   let name = &self.name.to_string().to_case(Case::Kebab);
    //   let attributes = &self.attributes;
    //   let children = &self.children;

    //   let mut attrs = vec![];

    //   for Attribute {
    //     extended: _,
    //     key,
    //     value,
    //   } in attributes.attrs.iter()
    //   {
    //     attrs.push(quote! {
    //       #key: #value
    //     });
    //   }

    //   if !children.nodes.is_empty() {
    //     attrs.push(quote!( children: ahecha::html::Node::Fragment( #children ) ))
    //   }

    //   let args = if attrs.is_empty() {
    //     quote!()
    //   } else {
    //     quote!( #ident ::ViewParams { #(#attrs,)* } )
    //   };

    //   let element = quote!(
    //     ahecha::html::Node::Element(ahecha::html::Element {
    //       attributes: #attributes,
    //       children: vec![ #ident ::view( #args ) ],
    //       name: #name,
    //     })
    //   );
    //   element.to_tokens(tokens);
    todo!("Implement ToTokens for HtmlCustomElement")
  }
}

impl ToString for HtmlCustomElement {
  fn to_string(&self) -> String {
    format!("<{}>...</{}>", self.name, self.name)
  }
}
