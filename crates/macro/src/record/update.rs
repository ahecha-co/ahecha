use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};

use super::{Field, Returning, WhereConstraint};

pub struct UpdateFieldsSet {
  pub fields: Vec<Field>,
}

impl UpdateFieldsSet {
  fn build(&self) -> (String, Vec<TokenStream>) {
    (
      self
        .fields
        .iter()
        .enumerate()
        .map(|(i, f)| {
          let name = &f.name;
          format!("{} = ${}", name, i + 1)
        })
        .collect::<Vec<_>>()
        .join(", "),
      self
        .fields
        .iter()
        .map(|f| {
          let name = &f.name;
          quote!( &self. #name )
        })
        .collect(),
    )
  }
}

pub struct UpdateStatement {
  pub span: Span,
  pub table_name: String,
  pub fields: UpdateFieldsSet,
  pub constraint: WhereConstraint,
  pub returning: Returning,
}

impl ToTokens for UpdateStatement {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let (sets, update_params) = self.fields.build();
    let (where_query, where_params) = self.constraint.build(update_params.len());
    let query = format!(
      "UPDATE {} SET {} {}{}",
      &self.table_name,
      sets,
      where_query,
      if self.returning.is_returning() {
        format!(" RETURNING {}", self.returning.build().join(", "))
      } else {
        "".to_owned()
      }
    );

    let fn_ident = if self.constraint.fields.is_empty() {
      Ident::new("insert", self.span)
    } else {
      Ident::new(
        format!(
          "update_where_{}",
          self
            .constraint
            .fields
            .iter()
            .map(|f| f.name.to_string())
            .collect::<Vec<_>>()
            .join("_and_"),
        )
        .as_str(),
        self.span,
      )
    };
    let fn_input = &self.constraint.fields;
    let query_params = [update_params, where_params].concat();

    dbg!(&query);

    if self.returning.is_returning() {
      quote!(
        pub async fn #fn_ident <DB, T> (&self, mut pool: &mut sqlx::Pool<DB> #(, #fn_input)*) -> Result<T, sqlx::Error> where DB: sqlx::Database {
          sqlx::query!(#query #(, #query_params)*).fetch_one(&mut pool).await
        }
      ).to_tokens(tokens);
    } else {
      quote!(
        pub async fn #fn_ident <DB> (&self, mut pool: &mut sqlx::Pool<DB> #(, #fn_input)*) -> Result<(), sqlx::Error> where DB: sqlx::Database {
          sqlx::query!(#query #(, #query_params)*).execute(&mut pool).await
        }
      ).to_tokens(tokens);
    }
  }
}
