use std::fmt::Error;

use proc_macro2::Ident;
use proc_macro_error::emit_error;
use quote::{quote, ToTokens};
use syn::{braced, bracketed, spanned::Spanned, ext::IdentExt, parse::Parse, parse_macro_input, Lit, LitStr, Token};

enum ParseError {
  Skip,
}

struct OrderBy;

enum Comparison {
  Between,
  Equal,
  GreaterThan,
  GreaterOrEqualThan,
  In,
  LessThan,
  LessOrEqualThan,
  Like,
}

enum Condition {
  And(Comparison),
  Group(Box<Condition>),
  Or(Comparison),
}

struct DeleteStatement {
  table: Ident,
  condition: Vec<Condition>,
  order: Vec<OrderBy>,
  limit: Option<usize>,
}

impl DeleteStatement {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    todo!()
  }
}

impl ToTokens for DeleteStatement {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    todo!()
  }
}

struct InsertStatement {
  table: Ident,
  data: Ident,
  // on_duplicate_key_update: Option<Ident>,
}

impl InsertStatement {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let data = syn::Ident::parse_any(input)?;
    let into = syn::Ident::parse_any(input)?;

    if into.to_string().to_lowercase() == "into" {
      let table = syn::Ident::parse_any(input)?;

      Ok(Self { table, data })
    } else {
      Err(syn::parse::Error::new(
        into.span(),
        format!("Expected token INTO, but found {}", into).as_str(),
      ))
    }
  }
}

impl ToTokens for InsertStatement {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let table = &self.table;
    let input = &self.data;
    quote!({
      ahecha::prepare_columns_statement!(
        "INSERT INTO {} ({}) VALUES ($) RETURNING id",
        #table ::name(),
        #input ::columns(),
      )
    })
    .to_tokens(tokens);
  }
}

struct SelectStatement {
  columns: Vec<Ident>,
  table: Ident,
  conditions: Vec<Condition>,
  order: Vec<OrderBy>,
  limit: Option<usize>,
}

impl SelectStatement {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    todo!()
  }
}

impl ToTokens for SelectStatement {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    todo!()
  }
}

struct UpdateStatement {
  table: Ident,
  columns: Vec<(Ident, Ident)>,
  conditions: Vec<Condition>,
  order: Vec<OrderBy>,
  limit: Option<usize>,
}

impl UpdateStatement {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    todo!()
  }
}

impl ToTokens for UpdateStatement {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    todo!()
  }
}

enum Statement {
  Delete(DeleteStatement),
  Insert(InsertStatement),
  Select(SelectStatement),
  Update(UpdateStatement),
}

impl Parse for Statement {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let statement = input.parse::<syn::Ident>()?;

    match statement.to_string().to_lowercase().as_str() {
      "delete" => Ok(Statement::Delete(DeleteStatement::parse(input)?)),
      "insert" => Ok(Statement::Insert(InsertStatement::parse(input)?)),
      "select" => Ok(Statement::Select(SelectStatement::parse(input)?)),
      "update" => Ok(Statement::Update(UpdateStatement::parse(input)?)),
      _ => Err(syn::parse::Error::new(
        statement.span(),
        format!(
          "Expected DELETE, INSERT, SELECT or UPDATE keyword but found {}",
          &statement.to_string()
        )
        .as_str(),
      )),
    }
  }
}

impl ToTokens for Statement {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    match self {
      Statement::Delete(q) => quote!(#q).to_tokens(tokens),
      Statement::Insert(q) => quote!(#q).to_tokens(tokens),
      Statement::Select(q) => quote!(#q).to_tokens(tokens),
      Statement::Update(q) => quote!(#q).to_tokens(tokens),
    }
  }
}

pub fn create_query(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let query = parse_macro_input!(input as Statement);
  quote!( #query ).into()
}

struct Column {
  ident: String,
  is_primary_key: bool,
  sqlx_cast_as_underscore: bool,
}

impl Parse for Column {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let name = input.parse::<syn::Ident>()?;

    if name.to_string() == "TableColumn" {
      let content;
      let mut ident = None;
      let mut is_primary_key = None;
      let mut sqlx_cast_as_underscore = None;

      braced!(content in input);

      while content.peek(syn::Ident) {
        let key = content.parse::<syn::Ident>()?;
        content.parse::<Token![:]>()?;
        let value = content.parse::<Lit>()?;
        content.parse::<Token![,]>()?;

        match key.to_string().as_str() {
          "ident" => match value {
            Lit::Str(value) => ident = Some(value.value()),
            _ => emit_error!(key.span(), "Expected a `&str`"),
          },
          "is_primary_key" => match value {
            Lit::Bool(bool) => {
              is_primary_key = Some(bool.value());
            }
            _ => emit_error!(key.span(), "Expected a `bool`"),
          },
          "sqlx_cast_as_underscore" => match value {
            Lit::Bool(bool) => {
              sqlx_cast_as_underscore = Some(bool.value);
            }
            _ => emit_error!(key.span(), "Expected a `bool`"),
          },
          _ => emit_error!(key.span(), "Unhandled key"),
        }
      }

      if let (Some(ident), Some(is_primary_key), Some(sqlx_cast_as_underscore)) =
        (ident, is_primary_key, sqlx_cast_as_underscore)
      {
        Ok(Self {
          ident,
          is_primary_key,
          sqlx_cast_as_underscore,
        })
      } else {
        Err(syn::parse::Error::new(
          name.span(),
          "Couldn't parse the TableColumn struct",
        ))
      }
    } else {
      Err(syn::parse::Error::new(
        name.span(),
        format!("Expected token `TableColumn`, but found {}", &name).as_str(),
      ))
    }
  }
}

struct PrepareColumnsStatement {
  sql: String,
  name: Ident,
  columns: Vec<Column>,
}

impl Parse for PrepareColumnsStatement {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let mut columns = vec![];
    dbg!(input);
    let sql = input.parse::<LitStr>()?.value();
    input.parse::<Token![,]>()?;
    let name = syn::Ident::parse_any(input)?;
    input.parse::<Token![,]>()?;
    let vec = input.parse::<syn::Ident>()?;
    if vec.to_string() == "vec" {
      input.parse::<Token![!]>()?;
      let content;
      bracketed!(content in input);
      while content.peek(syn::Ident) {
        let column = content.parse::<Column>()?;
        columns.push(column);
      }
    } else {
      return Err(syn::parse::Error::new(
        vec.span(),
        format!("Expected token `vec` but found {}", &vec).as_str(),
      ));
    }

    Ok(Self { sql, name, columns })
  }
}

impl ToTokens for PrepareColumnsStatement {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let sql = &self.sql;
    let col_numbers = self
      .columns
      .iter()
      .enumerate()
      .map(|(i, _)| {
        let out = format!("${}", i + 1);
        quote!(#out)
      })
      .collect::<Vec<_>>();
    let struct_cols = self
      .columns
      .iter()
      .filter_map(|c| {
        if c.is_primary_key {
          None
        } else {
          let ident = &c.ident;
          if c.sqlx_cast_as_underscore {
            Some(quote!( &self. #ident as _))
          } else {
            Some(quote!( &self. #ident ))
          }
        }
      })
      .collect::<Vec<_>>();

    dbg!(&sql, &col_numbers, &struct_cols);
    quote!(
      ahecha::prepare_statement!(format!(#sql, #(#col_numbers),*), #(#struct_cols),*)
    ).to_tokens(tokens);
  }
}

pub fn prepare_columns_statement(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let query = parse_macro_input!(input as PrepareColumnsStatement);
  quote!( #query ).into()
}

pub fn prepare_statement(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  dbg!(input);
  quote!().into()
}
