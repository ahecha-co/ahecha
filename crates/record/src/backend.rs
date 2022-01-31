use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream};

pub mod postgres;

pub struct TableIdent {
  ident: Ident,
}

pub enum ConditionStatement {
  Eq(Ident, Ident),
}

pub enum WhereStatement {
  Condition(ConditionStatement),
}

pub struct DeleteStatement;

impl Parse for DeleteStatement {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    todo!()
  }
}

pub struct InsertStatement;

impl Parse for InsertStatement {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    todo!()
  }
}

enum SelectType {
  All,
  Distinct,
}

impl Default for SelectType {
  fn default() -> Self {
    Self::All
  }
}

pub struct SelectStatement {
  select_type: SelectType,
  table_ident: TableIdent,
  where_statement: Option<Ident>,
  group_by_statement: Option<Ident>,
  having_statement: Option<Ident>,
  window_statement: Option<Ident>,
  order_statement: Option<Ident>,
  limit_statement: Option<Ident>,
  offset_statement: Option<Ident>,
  fetch_statement: Option<Ident>,
  for_statement: Option<Ident>,
}

impl Parse for SelectStatement {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let table_ident = input.parse::<Ident>()?;
    let first_char = {
      let chars = table_ident.to_string().chars().collect::<Vec<_>>();
      *chars.first().unwrap()
    };

    if first_char != first_char.to_uppercase().nth(0).unwrap() {
      return Err(syn::Error::new(
        table_ident.span(),
        "Expected a struct type",
      ));
    }

    Ok(Self {
      select_type: SelectType::All,
      table_ident: TableIdent { ident: table_ident },
      where_statement: None,
      group_by_statement: None,
      having_statement: None,
      window_statement: None,
      order_statement: None,
      limit_statement: None,
      offset_statement: None,
      fetch_statement: None,
      for_statement: None,
    })
  }
}

pub struct UpdateStatement;

impl Parse for UpdateStatement {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    todo!()
  }
}

enum Ast {
  Delete(DeleteStatement),
  Insert(InsertStatement),
  Select(SelectStatement),
  Update(UpdateStatement),
}

impl Parse for Ast {
  fn parse(input: ParseStream) -> syn::Result<Ast> {
    let ident = input.parse::<Ident>()?;
    match ident.to_string().to_uppercase().as_str() {
      "DELETE" => Ok(Ast::Delete(DeleteStatement::parse(input)?)),
      "INSERT" => Ok(Ast::Insert(InsertStatement::parse(input)?)),
      "SELECT" => Ok(Ast::Select(SelectStatement::parse(input)?)),
      "UPDATE" => Ok(Ast::Update(UpdateStatement::parse(input)?)),
      _ => Err(syn::Error::new(
        ident.span(),
        format!(
          "Expected DELETE, INSERT, SELECT or UPDATE token, but found `{}`",
          ident
        ),
      )),
    }
  }
}
