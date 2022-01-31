use std::fmt::Error;

/*
record!("users", {
  pub struct User {
    id, // infers type and primary key from the schema
    name,
    email, // infers from the schema that is indexed
    password: Password, // You can still specify the type if you want, for cases like this
    created_at: DateTime<Utc>, // infers from the schema that has default
    updated_at: DateTime<Utc>, // infers from the schema that has default
  }

  // This are called variants, and are checked all with the same rule as the first one
  struct UserSignup {
    name,
    email,
    password,
  }

  pub(crate) struct UserSession {
    id,
    name,
    email,
  }

  struct Author {
    id,
    name,
    posts: Vec<Post>,
  }
});

record!("posts", {
  pub struct Post {
    id,
    title,
    content,
    author_id,
    created_at,
    updated_at,
  }

  struct PostAuthor {
    id,
    title,
    content,
    author: UserSession,
  }
});

// -- Expands to
// >> Auxiliary files will be written that will be use by the query macro

pub struct User {
  id: Uuid,
  name: String,
  email: String,
  password: Password,
  created_at: DateTime<Utc>,
  updated_at: DateTime<Utc>,
}

impl Record for User {}

struct UserSignup {
  name: String,
  email: String,
  password: Password,
}

impl Record for UserSignup {}

pub(crate) struct UserSession {
  id: Uuid,
  name: String,
  email: String,
  password: Password,
}

impl Record for UserSession {}

struct Author {
  id: Uuid,
  name: String
  posts: Vec<Post>
}

impl Record for Author {}

pub struct Post {
  id,
  title,
  content,
  author_id,
  created_at,
  updated_at,
}

impl Record for Post {}

struct PostAuthor {
  id,
  title,
  content,
  author: UserSession,
}

impl Record for PostAuthor {}

// Reads the data from tmp/User.json? to do get the fields, table name, and check the fields at compile time
query!(SELECT User WHERE User.id = 'abc')

// It already knows that Author uses the "users" table
query!(SELECT Author WHERE Author::id = 'abc')
// `SELECT Author.id, Author.name, json_agg(Post.id, Post.title, ...) FROM users as Author Where Author.id = 'abc';
// > :warning check if json_agg can do what I expect

query!(SELECT Author WHERE Author::id = 'abc' LEFT JOIN Post ON Post::author_id = Author::id)
// `SELECT Author.id, Author.name, json_agg(Post.id, Post.title, ...) FROM users as Author Where Author.id = 'abc';

let post = Post { ... };
query!(INSERT Post VALUES (post))
>> `INSERT posts as Post (Post.id, Post.title, Post.content, ...) VALUES ($1, $2, ...)`, ...

let posts = vec![Post { ... }, Post { ... }];
query!(INSERT Post[] VALUES (post))
>> `INSERT posts as Post (Post.id, Post.title, Post.content, ...) VALUES ($1, $2, ...), ($n1, $n2, ...)`, ...

query!(UPDATE Post SET post WHERE Post::id)
>> `UPDATE posts as Post SET post.title = $1, post.content = $2, post.author_id = $3, post.created_at = $4, post.updated_at = $5
>>    WHERE Post.id = $6`, post.title, ..., post.id
// `id` is automatically excluded from the SET statement because is used in the WHERE statement

query!(UPDATE Post SET post WHERE Post::id AND Post::author_id)
>> `UPDATE posts as Post SET post.title = $1, post.content = $2, post.created_at = $3, post.updated_at = $4
>>    WHERE Post.id = $5 AND Post.author_id = $6`, post.title, ..., post.id, post.author_id
// `id` and `author_id` are automatically excluded from the SET statement because is used in the WHERE statement

query!(DELETE Post WHERE Post::id = { post.id })
>> `DELETE posts as Post WHERE Post.id = $1`, post.id

query!(SELECT NotARecord)
>> Will throw a rust error complaining about the `Record` trait not being implemented, if you do so manually it will complain about
// the missing entry in the offline schema, if it has the same name as an existing one it might work if the fields are the same, if not,
// you will get some kind of error, this is a limitation of the system, because we don't have all the information about the struct it could
// leat to some issues if multiple structs share the same name.
*/

use proc_macro2::Ident;
use proc_macro_error::emit_error;
use quote::{quote, ToTokens};
use syn::{
  braced, bracketed, ext::IdentExt, parse::Parse, parse_macro_input, spanned::Spanned, Lit, LitStr,
  Token,
};

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
    quote!(ahecha::prepare_statement!(format!(#sql, #(#col_numbers),*), #(#struct_cols),*))
      .to_tokens(tokens);
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
