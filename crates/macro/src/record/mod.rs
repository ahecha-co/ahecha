use ahecha_cli::config::{get_config, Column, TableConfig};
use proc_macro2::{Ident, Span, TokenStream};
use proc_macro_error::abort;
use quote::quote;
use quote::{spanned::Spanned, ToTokens};
use syn::{FieldsNamed, ItemStruct};

#[derive(PartialEq, Clone)]
enum Action {
  Delete,
  Insert,
  Update,
}

pub struct TableRecord {
  actions: Vec<Action>,
  config: TableConfig,
  fields: Vec<String>,
  ident: Ident,
  returning: Vec<String>,
  span: Span,
  table_name: String,
}

impl TableRecord {
  pub fn new(item: ItemStruct) -> Self {
    let span = item.__span();
    let ident = item.ident.clone();
    let fields = match item.fields {
      syn::Fields::Named(FieldsNamed {
        brace_token: _,
        named,
      }) => named
        .iter()
        .map(|f| f.ident.as_ref().unwrap().to_string())
        .collect::<Vec<_>>(),
      syn::Fields::Unnamed(_) => panic!("Unnamed fields not supported"),
      syn::Fields::Unit => panic!("Unit fields not supported"),
    };

    let (table_name, actions, returning) = if let Some(attr) = item.attrs.iter().find(|attr| {
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

    let config = match get_config().table(&table_name) {
      Some(config) => config,
      None => abort!(
        span,
        format!(
          "`{}` is not present in record.json, please add it and run `ahecha` cli if needed",
          &table_name
        )
      ),
    };

    Self {
      actions,
      config,
      fields,
      ident,
      returning,
      span,
      table_name,
    }
  }

  pub fn mod_ident(&self) -> Ident {
    Ident::new(
      format!("AhechaRecord{}", self.ident.to_string()).as_str(),
      self.span,
    )
  }

  pub fn mod_record(&self) -> TokenStream {
    let ident = self.mod_ident();
    let fields = self.returning.iter().map(|field_name| {
      let ident = Ident::new(field_name, self.span);
      let column = match self.config.columns.iter().find(|c| &c.name == field_name) {
        Some(column) => column,
        None => abort!(self.span, "There is no column with name `{}` found in `{}` in the records.json, check it or run `ahecha` cli again", field_name, self.table_name),
      };
      let ty = get_type_for_column(&column , self.span);
      quote!{ pub #ident : #ty }
    }).collect::<Vec<_>>();

    quote!(
      #[allow(non_snake_case)]
      pub mod #ident {
        pub struct Record {
         #(#fields),*
        }
      }
    )
    .into()
  }
}

impl ToTokens for TableRecord {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let delete_tokens = delete_stmt(&self);
    let insert_tokens = insert_stmt(&self);
    let update_tokens = update_stmt(&self);
    let ident = &self.ident;

    quote!(
      impl #ident {
        #delete_tokens
        #insert_tokens
        #update_tokens
      }
    )
    .to_tokens(tokens);
  }
}

fn delete_stmt(record: &TableRecord) -> TokenStream {
  if record.actions.contains(&Action::Delete) {
    let (_, where_args, fn_ident, fn_args) = where_parts(&record, "delete", 0);
    let query = delete_query(&record);

    quote!(
      pub async fn #fn_ident <'a, 'c: 'a>(pool: impl sqlx::Executor<'c, Database = sqlx::Postgres> + 'a, #(#fn_args),* ) -> Result<(), sqlx::Error> {
        sqlx::query!( #query, #(#where_args),* ).execute(pool).await?;
        Ok(())
      }
    )
  } else {
    quote!()
  }
}

fn delete_query(record: &TableRecord) -> String {
  let (where_stmt, _, _, _) = where_parts(&record, "delete", 0);
  format!("DELETE FROM {} {}", &record.table_name, where_stmt)
}

fn insert_stmt(record: &TableRecord) -> TokenStream {
  if record.actions.contains(&Action::Insert) {
    let query = insert_query(&record);
    let query_args = vec![
      record
        .fields
        .iter()
        .map(|f| {
          let ident = Ident::new(f, record.span);
          quote!( &self. #ident )
        })
        .collect::<Vec<_>>(),
      record
        .config
        .constraints
        .iter()
        .map(|f| {
          let ident = Ident::new(f, record.span);
          quote!( #ident )
        })
        .collect::<Vec<_>>(),
    ]
    .concat();

    let fn_ident = Ident::new(
      format!("insert_for_{}", record.config.constraints.join("_and_")).as_str(),
      record.span,
    );

    let fn_args = get_fn_args(&record, record.config.constraints.clone());

    if !record.returning.is_empty() {
      let query = format!("{} RETURNING {}", query, record.returning.join(", "));
      let mod_ident = record.mod_ident();

      quote!(
        pub async fn #fn_ident <'a, 'c: 'a>(&self, pool: impl sqlx::Executor<'c, Database = sqlx::Postgres> + 'a, #(#fn_args),* ) -> Result< #mod_ident::Record, sqlx::Error> {
          sqlx::query_as!( #mod_ident::Record, #query, #(#query_args),* ).fetch_one(pool).await
        }
      )
    } else {
      quote!(
        pub async fn #fn_ident <'a, 'c: 'a>(&self, pool: impl sqlx::Executor<'c, Database = sqlx::Postgres> + 'a, #(#fn_args),* ) -> Result<(), sqlx::Error> {
          sqlx::query!( #query, #(#query_args),* ).execute(pool).await?;
          Ok(())
        }
      )
    }
  } else {
    quote!()
  }
}

fn insert_query(record: &TableRecord) -> String {
  let fields = vec![record.fields.clone(), record.config.constraints.clone()].concat();
  format!(
    "INSERT INTO {} ({}) VALUES ({})",
    &record.table_name,
    fields.join(", "),
    fields
      .iter()
      .enumerate()
      .map(|(i, _)| format!("${}", i + 1))
      .collect::<Vec<_>>()
      .join(", ")
  )
}

fn update_stmt(record: &TableRecord) -> TokenStream {
  if record.actions.contains(&Action::Update) {
    let (_, where_args, fn_ident, fn_args) = where_parts(&record, "update", record.fields.len());
    let set_args = record
      .fields
      .iter()
      .map(|f| {
        let ident = Ident::new(f, record.span);
        quote!( self. #ident )
      })
      .collect::<Vec<_>>();
    let query = update_string(&record);
    let args = [set_args, where_args].concat();

    if !record.returning.is_empty() {
      let query = format!("{} RETURNING {}", query, record.returning.join(", "));
      let mod_ident = record.mod_ident();

      quote!(
        pub async fn #fn_ident <'a, 'c: 'a>(&self, pool: impl sqlx::Executor<'c, Database = sqlx::Postgres> + 'a, #(#fn_args),* ) -> Result< #mod_ident::Record, ::sqlx::Error>
        {
          sqlx::query_as!( #mod_ident::Record, #query, #(#args),* ).fetch_one(pool).await
        }
      )
    } else {
      quote!(
        pub async fn #fn_ident <'a, 'c: 'a>(&self, pool: impl sqlx::Executor<'c, Database = sqlx::Postgres> + 'a, #(#fn_args),* ) -> Result<(), sqlx::Error> {
          sqlx::query!( #query, #(#args),* ).execute(pool).await?;
          Ok(())
        }
      )
    }
  } else {
    quote!()
  }
}

fn update_string(record: &TableRecord) -> String {
  let (where_stmt, _, _, _) = where_parts(&record, "update", record.fields.len());

  let set = record
    .fields
    .iter()
    .enumerate()
    .map(|(i, f)| format!("{f} = ${}", i + 1))
    .collect::<Vec<_>>();

  format!(
    "UPDATE {} SET {} {}",
    &record.table_name,
    set.join(", "),
    where_stmt
  )
}

fn where_parts(
  record: &TableRecord,
  method_prefix: &str,
  index_offset: usize,
) -> (String, Vec<TokenStream>, Ident, Vec<TokenStream>) {
  let constraints = [
    record.config.primary_keys.clone(),
    record.config.constraints.clone(),
  ]
  .concat();
  if !constraints.is_empty() {
    let query = format!(
      "WHERE {}",
      constraints
        .iter()
        .enumerate()
        .map(|(i, f)| format!("{f} = ${}", i + 1 + index_offset))
        .collect::<Vec<_>>()
        .join(" AND ")
    );

    let query_args = constraints
      .iter()
      .map(|f| {
        let ident = Ident::new(f, record.span);
        quote!(#ident)
      })
      .collect::<Vec<_>>();

    let fn_ident = Ident::new(
      format!("{method_prefix}_where_{}", constraints.join("_and_")).as_str(),
      record.span,
    );

    let fn_args = get_fn_args(&record, constraints);

    (query, query_args, fn_ident, fn_args)
  } else {
    (
      "".to_owned(),
      vec![],
      Ident::new(method_prefix, record.span),
      vec![],
    )
  }
}

fn get_fn_args(record: &TableRecord, field_names: Vec<String>) -> Vec<TokenStream> {
  field_names.iter().map(|field_name| {
    let ident = Ident::new(field_name, record.span);
    let ty = match record.config.columns.iter().find(|c| &c.name == field_name) {
      Some(column) => get_type_for_column(&column, record.span),
      None => abort!(record.span, "There is no column with name `{}` found in `{}` in the records.json, check it or run `ahecha` cli again", field_name, record.table_name),
    };
    quote!( #ident : & #ty )
  }).collect()
}

// TODO: Hide behind feature flag and implement for each database
fn get_type_for_column(column: &Column, span: Span) -> TokenStream {
  let ty = match column.ty.as_str() {
    "varchar" => quote!(&str),
    "int4" => quote!(i32),
    "uuid" => quote!(ahecha::uuid::Uuid),
    _ => abort!(
      span,
      "The type `{}` is not mapped to a rust type yet, please open an issue or PR",
      &column.ty
    ),
  };

  if column.is_nullable {
    quote!( Option< #ty > )
  } else {
    ty
  }
}

fn parse_attr_args(tokens: TokenStream) -> (Option<String>, Vec<Action>, Vec<String>) {
  let mut actions = vec![];
  let mut returning = vec![];
  let mut table_name = None;

  let mut iter = tokens.into_iter();
  while let Some(token) = iter.next() {
    match token.clone() {
      proc_macro2::TokenTree::Group(group) => abort!(group, "Unsupported group `{}`", group),
      proc_macro2::TokenTree::Ident(ident) => match ident.to_string().as_str() {
        "deleteable" | "deletable" => actions.push(Action::Delete),
        "insertable" => actions.push(Action::Insert),
        "updateable" | "updatable" => actions.push(Action::Update),
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

  return (table_name, actions, returning);
}

#[cfg(test)]
mod test {
  use super::*;
  use ahecha_cli::config::Column;

  fn test_record() -> TableRecord {
    TableRecord {
      actions: vec![],
      config: TableConfig {
        columns: vec![
          Column {
            name: "id".to_string(),
            ty: "uuid".to_string(),
            is_nullable: false,
          },
          Column {
            name: "name".to_string(),
            ty: "varchar".to_string(),
            is_nullable: false,
          },
          Column {
            name: "age".to_string(),
            ty: "int4".to_string(),
            is_nullable: true,
          },
          Column {
            name: "tenant_id".to_string(),
            ty: "varchar".to_string(),
            is_nullable: false,
          },
        ],
        primary_keys: vec!["id".to_string()],
        constraints: vec!["tenant_id".to_string()],
      },
      fields: vec!["name".to_string(), "age".to_string()],
      ident: Ident::new("TestRecord", Span::call_site()),
      returning: vec!["id".to_string()],
      span: Span::call_site(),
      table_name: "users".to_owned(),
    }
  }

  #[test]
  fn test_simple_delete_string_query() {
    let query = delete_query(&test_record());
    assert_eq!("DELETE FROM users WHERE id = $1 AND tenant_id = $2", query);
  }

  #[test]
  fn test_simple_insert_string_query() {
    let query = insert_query(&test_record());
    assert_eq!(
      "INSERT INTO users (name, age, tenant_id) VALUES ($1, $2, $3)",
      query
    );
  }

  #[test]
  fn test_simple_update_string_query() {
    let query = update_string(&test_record());
    assert_eq!(
      "UPDATE users SET name = $1, age = $2 WHERE id = $3 AND tenant_id = $4",
      query
    );
  }

  #[test]
  fn test_simple_returning_tokenstream() {
    let mut config = test_record();
    config.returning = vec!["age".to_string()];
    let tokens = config.mod_record();
    assert_eq!("", tokens.to_string());
  }
}
