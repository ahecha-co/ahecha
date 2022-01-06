use std::collections::HashMap;

use proc_macro2::{Ident, TokenStream};
use proc_macro_error::emit_error;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, punctuated::Punctuated, Fields, FieldsNamed, ItemStruct};

use self::{length::LengthValidator, required::RequiredValidator};

// mod date;
// mod datetime;
// mod email;
mod length;
// mod phone;
// mod range;
mod required;
// mod time;
// mod url;

pub(crate) struct ValidatorAttribute {
  name: Ident,
  attrs: HashMap<String, String>,
}

impl ValidatorAttribute {
  fn get(&self, key: &str) -> Option<String> {
    if let Some(value) = self.attrs.get(key) {
      Some(value.clone())
    } else {
      None
    }
  }

  fn allowed_attrs(&self, attrs_keys: Vec<&str>) {
    let unsupported_keys = self
      .attrs
      .keys()
      .filter(|key| attrs_keys.contains(&key.as_str()))
      .map(|k| k.clone())
      .collect::<Vec<_>>();

    emit_error!(
      self.name.span(),
      "They attributes {} are not supported.",
      unsupported_keys.join(",")
    );
  }

  // fn from(attr: &Attribute) -> Option<Self> {
  //   let ident = attr.path.segments.iter().last().unwrap().ident.clone();
  //   // Skip other attributes
  //   if ident.to_string() == "validate" {
  //     let mut name = None;
  //     match attr.parse_meta() {
  //       Ok(syn::Meta::List(syn::MetaList { ref nested, .. })) => {
  //         let nested_meta = nested.iter().map(|n| n).collect::<Vec<_>>();
  //         dbg!();
  //       }
  //       Ok(syn::Meta::Path(path)) => {
  //         if let Some(segment) = path.segments.iter().last() {
  //           name = Some(segment.ident.clone());
  //         }
  //       }
  //       Ok(_) => emit_error!(ident.span(), "Unsuported format"),
  //       Err(err) => emit_error!(ident.span(), "{}", err),
  //     }

  //     if let Some(name) = name {
  //       Some(Self {
  //         name,
  //         attrs: HashMap::new(),
  //       })
  //     } else {
  //       emit_error!(ident.span(), "Unknown attribute {}", ident.to_string());
  //       None
  //     }
  //   } else {
  //     None
  //   }
  // }
}

enum Validator {
  // Date(DateValidator),
  // DateTime(DateTimeValidator),
  // Email(EmailValidator),
  Length(LengthValidator),
  // Phone(PhoneValidator),
  // Range(RangeValidator),
  Required(RequiredValidator),
  // Time(TimeValidator),
  // Url(UrlValidator),
}

impl Validator {
  fn from(field: &syn::Field) -> Vec<Self> {
    field
      .attrs
      .iter()
      .flat_map(|attr| Self::from_attr(&field.ident.clone().unwrap(), attr))
      .collect::<Vec<_>>()
  }

  fn from_attr(field: &Ident, attr: &syn::Attribute) -> Vec<Self> {
    let validate = &attr.path.segments.iter().last().unwrap().ident;
    if validate.to_string() == "validate" {
      match attr.parse_meta() {
        Ok(syn::Meta::List(syn::MetaList { ref nested, .. })) => {
          Self::from_nested_meta(&validate.span(), &field, nested)
        }
        Ok(_) => {
          emit_error!(validate.span(), "Unsuported format");
          vec![]
        }
        Err(err) => {
          emit_error!(validate.span(), "{}", err);
          vec![]
        }
      }
    } else {
      vec![]
    }
  }

  fn from_nested_meta(
    span: &proc_macro2::Span,
    field: &Ident,
    nested: &syn::punctuated::Punctuated<syn::NestedMeta, syn::token::Comma>,
  ) -> Vec<Self> {
    nested
      .iter()
      .filter_map(|meta| match meta {
        syn::NestedMeta::Meta(meta) => match meta {
          syn::Meta::Path(path) => {
            let ident = &path.segments.iter().last().unwrap().ident;
            match ident.to_string().as_str() {
              "required" => Some(vec![Self::Required(RequiredValidator::new(
                field.clone(),
                None,
              ))]),
              _ => {
                emit_error!(
                  ident.span(),
                  "{} isn't a supported validator",
                  ident.to_string()
                );
                None
              }
            }
          }
          syn::Meta::List(list) => None,
          syn::Meta::NameValue(named) => {
            let span = named.path.segments.iter().last().unwrap().ident.span();
            emit_error!(span, "Unsuported named value at this level");
            None
          }
        },
        _ => {
          emit_error!(span, "Literals are not supported");
          None
        }
      })
      .flatten()
      .collect::<Vec<_>>()
  }
}

// impl From<(&Ident, &Attribute)> for Validator {
//   fn from((ident, attr): (&Ident, &Attribute)) -> Self {
//     if let Some(v_attr) = ValidatorAttribute::from(attr) {
//       match v_attr.name.to_string().as_str() {
//         // "date" => Self::Date(DateValidator::new(ident.clone(), v_attr)),
//         // "datetime" => Self::DateTime(DateTimeValidator::new(ident.clone(), v_attr)),
//         // "email" => Self::Email(EmailValidator::new(ident.clone(), v_attr)),
//         "length" => Self::Length(LengthValidator::new(ident.clone(), v_attr)),
//         // "phone" => Self::Phone(PhoneValidator::new(ident.clone(), v_attr)),
//         // "range" => Self::Range(RangeValidator::new(ident.clone(), v_attr)),
//         "required" => Self::Required(RequiredValidator::new(ident.clone(), v_attr)),
//         // "time" => Self::Time(TimeValidator::new(ident.clone(), v_attr)),
//         // "url" => Self::Url(UrlValidator::new(ident.clone(), v_attr)),
//         _ => {
//           emit_error!(
//             ident.span(),
//             "`{}` isn't a supported validator",
//             ident.to_string()
//           );
//           Self::None
//         }
//       }
//     } else {
//       Self::None
//     }
//   }
// }

impl ToTokens for Validator {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    match self {
      // Self::Date(value) => value.to_tokens(tokens),
      // Self::DateTime(value) => value.to_tokens(tokens),
      // Self::Email(value) => value.to_tokens(tokens),
      Self::Length(value) => value.to_tokens(tokens),
      // Self::Phone(value) => value.to_tokens(tokens),
      // Self::Range(value) => value.to_tokens(tokens),
      Self::Required(value) => value.to_tokens(tokens),
      // Self::Time(value) => value.to_tokens(tokens),
      // Self::Url(value) => value.to_tokens(tokens),
    }
  }
}

struct ValidatorBuilder {
  fields: Punctuated<syn::Field, syn::token::Comma>,
}

impl ToTokens for ValidatorBuilder {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let mut validators = vec![];

    for field in self.fields.iter() {
      for validator in Validator::from(field) {
        validators.push(validator);
      }
    }

    quote!(#(#validators)*).to_tokens(tokens);
  }
}

pub fn create_validator(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let struct_item = parse_macro_input!(input as ItemStruct);
  let name = &struct_item.ident;

  let validators = match &struct_item.fields {
    Fields::Named(FieldsNamed { named, .. }) => {
      let builder = ValidatorBuilder {
        fields: named.clone(),
      };
      quote!(#builder)
    }
    _ => {
      emit_error!(
        struct_item.ident.span(),
        "Only named properties are supported"
      );

      quote!()
    }
  };

  // let from_value = match &struct_item.fields {
  //   Fields::Named(FieldsNamed { named, .. }) => named
  //     .iter()
  //     .map(|f| {
  //       let ident = f.ident.clone().unwrap();
  //       let ident_str = ident.to_string();
  //       quote!(#ident: value[#ident_str].into() )
  //     })
  //     .collect::<Vec<TokenStream>>(),
  //   _ => {
  //     emit_error!(
  //       struct_item.ident.span(),
  //       "Only named properties are supported"
  //     );

  //     vec![]
  //   }
  // };

  quote!(
    impl ::ahecha::Validate for #name {
      fn validate(value: serde_json::Value) -> anyhow::Result<()> {
        let mut errors = vec![];

        #validators

        if errors.is_empty() {
          // Ok(Self {
          //   #(#from_value),*
          // })
          Ok()
        } else {
          let err = std::collections::HashMap::new();
          errors.iter().for_each(|(k, v)| err.insert(k, v));
          Err(ahecha::validate::Error::KeyValue(err))
        }
      }
    }
  )
  .into()
}
