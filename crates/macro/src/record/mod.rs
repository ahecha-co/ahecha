mod config;
mod delete;
mod insert;
mod update;

use std::{fmt::Debug, str::FromStr};

use proc_macro2::{Span, TokenStream};
use proc_macro_error::abort;
use quote::{quote, spanned::Spanned, ToTokens};
use syn::{parse_macro_input, FieldsNamed, Ident, ItemStruct, Type};

use self::{config::get_config, delete::*, insert::*, update::*};

#[derive(Clone)]
pub struct Field {
  name: Ident,
  ty: Type,
}

impl Debug for Field {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "Field {{ name: {}, ty: {} }}",
      &self.name,
      &self.ty.to_token_stream(),
    )
  }
}

impl ToTokens for Field {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let name = &self.name;
    let ty = &self.ty;
    quote!(#name: #ty).to_tokens(tokens);
  }
}

pub struct Returning {
  fields: Vec<Field>,
}

impl Returning {
  fn is_returning(&self) -> bool {
    !self.fields.is_empty()
  }

  fn build(&self) -> Vec<String> {
    self.fields.iter().map(|f| format!("{}", &f.name)).collect()
  }
}

pub struct WhereConstraint {
  fields: Vec<Field>,
}

impl WhereConstraint {
  fn build(&self, index_offset: usize) -> (String, Vec<TokenStream>) {
    let constraints = self
      .fields
      .iter()
      .enumerate()
      .map(|(i, f)| format!("{} = ${}", &f.name, i + index_offset + 1))
      .collect::<Vec<_>>();

    (
      if !constraints.is_empty() {
        format!("WHERE {}", constraints.join(" AND "))
      } else {
        "".to_owned()
      },
      self
        .fields
        .iter()
        .map(|f| {
          let name = &f.name;
          quote!( & #name )
        })
        .collect(),
    )
  }
}

#[derive(PartialEq, Clone)]
enum Action {
  Delete,
  Insert,
  Update,
}

#[derive(Clone)]
pub struct Record {
  actions: Vec<Action>,
  span: Span,
  table_name: String,
  fields: Vec<Field>,
  constraint: Vec<Field>,
  returning: Vec<Field>,
}

impl Record {
  pub fn new(item: ItemStruct) -> Self {
    let config = get_config();
    let span = item.__span();
    let fields = match item.fields {
      syn::Fields::Named(FieldsNamed {
        brace_token: _,
        named,
      }) => named
        .iter()
        // .map(|f| {
        //   let tokens = TokenStream::from_str(&f.ty).expect("{} is not a valid Type", &f.ty);
        //   Field {
        //     name: f.ident.as_ref().unwrap().clone(),
        //     ty: parse_macro_input!(tokens as Type),
        //   }
        // })
        .map(|f| Field {
          name: f.ident.as_ref().unwrap().clone(),
          ty: f.ty.clone(),
        })
        .collect::<Vec<_>>(),
      syn::Fields::Unnamed(_) => panic!("Unnamed fields not supported"),
      syn::Fields::Unit => panic!("Unit fields not supported"),
    };

    let (table_name, _constraint, actions, returning) = if let Some(attr) =
      item.attrs.iter().find(|attr| {
        attr
          .path
          .segments
          .iter()
          .find(|s| s.ident.to_string() == "record")
          .is_some()
      }) {
      let tokens = attr
        .tokens
        .clone()
        .into_iter()
        .map(|t| t)
        .collect::<Vec<_>>();
      if tokens.is_empty() {
        abort!(attr.__span(), "Expected record attributes");
      } else if tokens.len() > 1 {
        abort!(
          attr.__span(),
          "Expected record attributes, but got {:?}",
          tokens
        );
      } else {
        match tokens.first().unwrap() {
          proc_macro2::TokenTree::Group(g) => parse_attr_args(g.stream()),
          _ => abort!(attr.__span(), "Expected attributes, but got {:?}", tokens),
        }
      }
    } else {
      abort!(span, "Missing the `#[record()]` attribute")
    };

    let table_name = match table_name {
      Some(table_name) => table_name,
      None => abort!(span, r#"the #[record(table = "table_name")] is missing"#),
    };

    let (constraint, returning) = match config.table(&table_name) {
      Some(table) => (
        table
          .constraints
          .clone()
          .iter()
          .map(|f| Field {
            name: Ident::new(&f.name, span),
            ty: {
              let tokens = TokenStream::from_str(&f.ty)
                .expect(format!("{} is not a valid Type", &f.ty).as_str());
              parse_macro_input!(tokens as Type)
              // let ty = f
              //   .ty
              //   .split("::")
              //   .map(|v| Ident::new(&v, span))
              //   .collect::<Vec<_>>();
              // quote!( #(#ty)::* )
            },
          })
          .collect::<Vec<_>>(),
        table
          .columns
          .clone()
          .iter()
          .filter(|f| returning.contains(&f.name))
          .map(|f| Field {
            name: Ident::new(&f.name, span),
            ty: {
              let tokens = TokenStream::from_str(&f.ty)
                .expect(format!("{} is not a valid Type", &f.ty).as_str());
              parse_macro_input!(tokens as Type)
              // let ty = f
              //   .ty
              //   .split("::")
              //   .map(|v| Ident::new(&v, span))
              //   .collect::<Vec<_>>();
              // quote!( #(#ty)::* )
            },
          })
          .collect::<Vec<_>>(),
      ),
      None => (vec![], vec![]),
    };

    Self {
      actions,
      span,
      table_name,
      fields,
      constraint,
      returning,
    }
  }
}

impl ToTokens for Record {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    if self.actions.contains(&Action::Delete) {
      let Record {
        table_name,
        constraint,
        ..
      } = self.clone();
      DeleteStatement {
        span: self.span,
        table_name,
        constraint: WhereConstraint { fields: constraint },
      }
      .to_tokens(tokens);
    }

    if self.actions.contains(&Action::Insert) {
      let Record {
        table_name,
        fields,
        constraint,
        returning,
        ..
      } = self.clone();
      InsertStatement {
        span: self.span,
        table_name,
        constraint: InsertConstraint {
          fields: constraint.clone(),
        },
        fields: InsertFieldsValues {
          fields: [constraint, fields].concat(),
        },
        returning: Returning { fields: returning },
      }
      .to_tokens(tokens);
    }

    if self.actions.contains(&Action::Update) {
      let Record {
        table_name,
        fields,
        constraint,
        returning,
        ..
      } = self.clone();
      UpdateStatement {
        span: self.span,
        table_name,
        constraint: WhereConstraint { fields: constraint },
        fields: UpdateFieldsSet { fields },
        returning: Returning { fields: returning },
      }
      .to_tokens(tokens);
    }
  }
}

fn parse_attr_args(tokens: TokenStream) -> (Option<String>, Vec<Field>, Vec<Action>, Vec<String>) {
  let mut constraint = vec![];
  let mut actions = vec![];
  let mut returning = vec![];
  let mut table_name = None;

  let mut iter = tokens.into_iter();
  while let Some(token) = iter.next() {
    match token.clone() {
      proc_macro2::TokenTree::Group(group) => abort!(group, "Unsupported group `{}`", group),
      proc_macro2::TokenTree::Ident(ident) => match ident.to_string().as_str() {
        "deleteable" => actions.push(Action::Delete),
        "insertable" => actions.push(Action::Insert),
        "updateable" => actions.push(Action::Update),
        "table" => {
          if let Some(table_token) = iter.next() {
            if let proc_macro2::TokenTree::Punct(p) = table_token {
              match p.as_char() {
                '=' => {}
                _ => abort!(
                  token,
                  "Expected table name assigment, but found {}. Example: #[record(table=\"user\")",
                  p.as_char(),
                ),
              }
            } else {
              abort!(
                token,
                "Expected table name assigment, but found {}. Example: #[record(table=\"user\")",
                table_token,
              );
            }
          } else {
            abort!(
              token,
              "Expected table name. Example: #[record(table=\"user\")"
            )
          }

          if let Some(table_token) = iter.next() {
            if let proc_macro2::TokenTree::Literal(lit) = table_token {
              table_name = Some(lit.to_string().replace('"', ""));
            } else {
              abort!(
                token,
                "Expected table name, but found {}. Example: #[record(table=\"user\")",
                table_token,
              );
            }
          } else {
            abort!(
              token,
              "Expected table name. Example: #[record(table=\"user\")"
            )
          }
        }
        "constraint" => {
          if let Some(constraint_token) = iter.next() {
            if let proc_macro2::TokenTree::Group(g) = constraint_token {
              let mut stream_iter = g.stream().into_iter();
              while let Some(fn_arg_token) = stream_iter.next() {
                // FnArg ident
                let name = match fn_arg_token.clone() {
                  proc_macro2::TokenTree::Group(item) => {
                    abort!(item, "Expected ident, but found `{}`", item)
                  }
                  proc_macro2::TokenTree::Ident(item) => item,
                  proc_macro2::TokenTree::Punct(item) => {
                    abort!(item, "Expected ident, but found `{}`", item)
                  }
                  proc_macro2::TokenTree::Literal(item) => {
                    abort!(item, "Expected ident, but found `{}`", item)
                  }
                };

                // Colon
                match stream_iter.next() {
                  Some(colon_token) => match colon_token {
                    proc_macro2::TokenTree::Group(item) => {
                      abort!(item, "Expected `:`, but found `{}`", item)
                    }
                    proc_macro2::TokenTree::Ident(item) => {
                      abort!(item, "Expected `:`, but found `{}`", item)
                    }
                    proc_macro2::TokenTree::Punct(item) => match item.as_char() {
                      ':' => {}
                      _ => abort!(item, "Expected `:`, but found `{}`", item.as_char()),
                    },
                    proc_macro2::TokenTree::Literal(item) => {
                      abort!(item, "Expected `:`, but found `{}`", item)
                    }
                  },
                  None => abort!(fn_arg_token, "Expected `:`, but found nothing"),
                }

                // FnArg type
                // TODO: improve it to support `type::Path`, right now it only support ident
                let mut ty_tokens = vec![];
                while let Some(fn_arg_type_token) = stream_iter.next() {
                  match fn_arg_type_token {
                    proc_macro2::TokenTree::Group(item) => {
                      abort!(item, "Expected an ident, but found `{}`", item)
                    }
                    proc_macro2::TokenTree::Ident(item) => ty_tokens.push(item.to_token_stream()),
                    proc_macro2::TokenTree::Punct(item) => match item.as_char() {
                      ':' => ty_tokens.push(quote!( : )),
                      ',' => break,
                      _ => abort!(item, "Expected an ident, but found `{}`", item),
                    },
                    proc_macro2::TokenTree::Literal(item) => {
                      abort!(item, "Expected an ident, but found `{}`", item)
                    }
                  }
                }

                if ty_tokens.is_empty() {
                  abort!(fn_arg_token, "Expected an ident, but found nothing")
                }

                // If we got here, name and type are set.
                constraint.push(Field {
                  name,
                  ty: quote!( #(#ty_tokens)* ),
                });
              }
            } else {
              abort!(
                constraint_token,
                "Expected field and type list, but got `{}`",
                constraint_token
              )
            }
          } else {
            abort!(
              token,
              "Expected constraint field and type list,but got nothing"
            )
          }
        }
        "returning" => {
          if let Some(token) = iter.next() {
            if let proc_macro2::TokenTree::Group(g) = token {
              let mut stream_iter = g.stream().into_iter();
              while let Some(ident_list_token) = stream_iter.next() {
                let ident = match ident_list_token {
                  proc_macro2::TokenTree::Group(item) => {
                    abort!(item, "Expected ident, but found `{}`", item)
                  }
                  proc_macro2::TokenTree::Ident(item) => item.to_string(),
                  proc_macro2::TokenTree::Punct(item) => {
                    abort!(item, "Expected ident, but found `{}`", item)
                  }
                  proc_macro2::TokenTree::Literal(item) => {
                    abort!(item, "Expected ident, but found `{}`", item)
                  }
                };

                // Maybe comma
                match iter.next() {
                  Some(comma_token) => match comma_token {
                    proc_macro2::TokenTree::Group(item) => {
                      abort!(item, "Expected `,`, but found `{}`", item)
                    }
                    proc_macro2::TokenTree::Ident(item) => {
                      abort!(item, "Expected `,`, but found `{}`", item)
                    }
                    proc_macro2::TokenTree::Punct(item) => match item.as_char() {
                      ',' => {}
                      _ => abort!(item, "Expected `,`, but found `{}`", item.as_char()),
                    },
                    proc_macro2::TokenTree::Literal(item) => {
                      abort!(item, "Expected `,`, but found `{}`", item)
                    }
                  },
                  None => {}
                }

                // If we got here, name and type are set.
                returning.push(ident);
              }
            } else {
              abort!(token, "Expected field and type list, but got `{}`", token)
            }
          } else {
            abort!(
              token,
              "Expected constraint field and type list,but got nothing"
            )
          }
        }
        _ => abort!(&ident, "`{}` is not supported", &ident),
      },
      proc_macro2::TokenTree::Punct(_) => continue,
      proc_macro2::TokenTree::Literal(lit) => abort!(lit, "Unsupported literal `{}`", lit),
    }
  }

  return (table_name, constraint, actions, returning);
}
