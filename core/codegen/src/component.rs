use convert_case::{Case, Casing};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{Field, ItemStruct, Type};

// use convert_case::{Case, Casing};
// use syn::ItemStruct;

// struct HTMLElement;

// trait Component<A> {
//   /// An instance of the element is created or upgraded. Useful for initializing state, setting up event listeners, or creating a shadow dom. See the spec for restrictions on what you can do in the constructor.
//   fn new() -> Self {
//     Self::default()
//   }
//   /// Called every time the element is inserted into the DOM. Useful for running setup code, such as fetching resources or rendering. Generally, you should try to delay work until this time.
//   fn connected_callback(&mut self) {}
//   /// Called every time the element is removed from the DOM. Useful for running clean up code.
//   fn disconnected_callback(&mut self) {}
//   /// Called when an observed attribute has been added, removed, updated, or replaced. Also called for initial values when an element is created by the parser, or upgraded. Note: only attributes listed in the observed_attributes property will receive this callback.
//   fn attribute_change_callback(&mut self, attribute: A) {}
//   /// The custom element has been moved into a new document (e.g. someone called document.adopt_node(el)).
//   fn adopted_callback(&self, el: HTMLElement) {}

//   fn render(&self) -> Html {}
// }

// pub struct CustomElement<T, OA, C: Component<OA>> {
//   name: String,
//   attributes: T,
//   observed_attributes: OA,
//   component: C,
// }

// #[cfg(feature = "backend")]
// impl CustomElement<T, OA, C: Component<OA>> {
//   pub fn new(name: String, attributes: T, observed_attributes: OA, component: C) -> Self {
//     component.connected_callback();

//     Self {
//       name,
//       attribute,
//       observed_attributes,
//       component,
//     }
//   }

//   pub fn render(&self) -> String {
//     self.component.render().to_string()
//   }
// }

// #[cfg(feature = "frontend")]
// impl CustomElement<T, OA, C: Component<OA>> {
//   pub fn new(name: String, attributes: T, observed_attributes: OA, component: C) -> Self {
//     let el = Self {
//       name: name.clone(),
//       attribute,
//       observed_attributes,
//       component,
//     };

//     el.define(name);

//     el
//   }

//   // Autogenerate this method to pass the value to the component, the idea is that AO is an enum with
//   // two values old and new converted to their respective types.
//   // Example:
//   //
//   // ```rust
//   // enum MyComponentObservedAttribute {
//   //   StrProp(Option<String>, Option<String>)
//   //   BoolProp(Option<bool>, Option<bool>)
//   //   IntProp(Option<int32>, Option<int32>)
//   //   StructProp(Option<CustomStruct>, Option<CustomStruct>) // derive serde serialize/deserialize
//   // }
//   // ```
//   fn attribute_change_callback(
//     &mut self,
//     name: String,
//     old_value: Option<String>,
//     new_value: Option<String>,
//   ) {
//   }

//   pub fn render(&self) -> String {
//     self.component.render().to_string()
//   }
// }

// pub fn component_from_struct(item: ItemStruct) {
//   // item.fields
//   // CustomElement {
//   //   name: item.ident.to_string().to_case(Case::Snake),
//   // }
// }

pub(crate) struct ComponentBuilder {
  ident: Ident,
  tag_name: String,
  fields: Vec<Field>,
}

impl ComponentBuilder {
  pub(crate) fn new(item_struct: &ItemStruct) -> Self {
    let ident = item_struct.ident.clone();
    let tag_name = ident.to_string().to_case(Case::Kebab);
    let mut fields = item_struct
      .fields
      .iter()
      .map(|f| f.clone())
      .collect::<Vec<_>>();

    // TODO: Check for other fields as store: GlobalStore<T>, state: State<S>, etc
    if fields
      .iter()
      .filter_map(|field| field.ident.clone())
      .find(|ident| ident.to_string() == "children")
      .is_none()
    {
      fields.push(Field {
        attrs: vec![],
        vis: syn::Visibility::Inherited,
        ident: Some(Ident::new("children", Span::call_site())),
        colon_token: None,
        ty: Type::Verbatim(quote!(Option<C>).into()),
      });
    }

    if fields
      .iter()
      .filter_map(|field| field.ident.clone())
      .find(|ident| ident.to_string() == "tag_name")
      .is_none()
    {
      fields.push(Field {
        attrs: vec![],
        vis: syn::Visibility::Inherited,
        ident: Some(Ident::new("tag_name", Span::call_site())),
        colon_token: None,
        ty: Type::Verbatim(quote!(&'static str).into()),
      });
    }

    Self {
      ident,
      tag_name,
      fields,
    }
  }

  pub fn get_fields_declaration(&self) -> TokenStream {
    let declaration_fields: Vec<TokenStream> = self
      .fields
      .iter()
      .map(|f| {
        quote! {
          #f
        }
        .into()
      })
      .collect();

    quote! {
      #(#declaration_fields),*
    }
    .into()
  }

  pub fn implementations(&self) -> TokenStream {
    let out = vec![self.impl_component(), self.impl_default()];

    quote! { #(#out)* }.into()
  }

  fn impl_component(&self) -> TokenStream {
    let ident = &self.ident;
    let out: Vec<TokenStream> = self
      .fields
      .iter()
      .filter(|f| match f.vis {
        syn::Visibility::Public(_) => f.ident.clone().unwrap().to_string() != "children",
        _ => false,
      })
      .map(|f| {
        let field_ident = f.ident.clone().unwrap();
        let field_str = field_ident.to_string();

        quote! {
          write!(writer, " {}=\"", #field_str)?;
          etagere::view::escape_html(&self.#field_ident, writer)?;
          write!(writer, "\"")?;
        }
        .into()
      })
      .collect();

    quote! {
      impl<C: etagere::view::ToHtml> etagere::view::ToHtml for #ident<C> {
        fn html_into<W: std::fmt::Write>(self, writer: &mut W) -> std::fmt::Result {
          match self.children {
            None => {
              write!(writer, "<{}", self.tag_name)?;
              #(#out)*
              write!(writer, "/>")
            }
            Some(renderable) => {
              write!(writer, "<{}", self.tag_name)?;
              #(#out)*
              write!(writer, ">")?;
              renderable.html_into(writer)?;
              write!(writer, "</{}>", self.tag_name)
            }
          }
        }
      }
    }
    .into()
  }

  fn impl_default(&self) -> TokenStream {
    let ident = &self.ident;
    let defaults: Vec<TokenStream> = self
      .fields
      .iter()
      .map(|f| {
        let field_ident = f.ident.clone().unwrap();
        let field_str = field_ident.to_string();
        let tag_name = &self.tag_name;

        if field_str == "tag_name" {
          quote! {
            #field_ident: #tag_name
          }
        } else {
          quote! {
            #field_ident: Default::default()
          }
        }
        .into()
      })
      .collect();

    quote! {
      impl<C: etagere::view::ToHtml> Default for #ident<C> {
        fn default() -> Self {
          Self {
            #(#defaults),*
          }
        }
      }

    }
    .into()
  }
}
