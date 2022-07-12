use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};

use super::{Field, Returning};

pub struct InsertFieldsValues {
  pub fields: Vec<Field>,
}

impl InsertFieldsValues {
  fn build(&self) -> (Vec<String>, Vec<String>, Vec<TokenStream>) {
    (
      self
        .fields
        .iter()
        .map(|f| format!("{}", f.name.to_string()))
        .collect(),
      (0..self.fields.len())
        .map(|v| format!("${}", v + 1))
        .collect(),
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

pub struct InsertConstraint {
  pub fields: Vec<Field>,
}

pub struct InsertStatement {
  pub span: Span,
  pub table_name: String,
  pub constraint: InsertConstraint,
  pub fields: InsertFieldsValues,
  pub returning: Returning,
}

impl ToTokens for InsertStatement {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let (columns, values, insert_params) = self.fields.build();
    let query = format!(
      "INSERT INTO {} ({}) VALUES ({}){};",
      &self.table_name,
      columns.join(", "),
      values.join(", "),
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
          "insert_where_{}",
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

    let query_params = [
      insert_params,
      self
        .constraint
        .fields
        .iter()
        .map(|f| {
          let name = &f.name;
          quote!( & #name )
        })
        .collect::<Vec<_>>(),
    ]
    .concat();

    if self.returning.is_returning() {
      quote!(
        pub async fn #fn_ident <T> (&self, pool: &mut <sqlx::Postgres as sqlx::Database>::Connection #(, #fn_input)*) -> Result<T, sqlx::Error> {
          sqlx::query!(#query #(, #query_params)*).fetch_one(pool).await
        }
      ).to_tokens(tokens);
    } else {
      quote!(
        pub async fn #fn_ident (&self, pool: &mut <sqlx::Postgres as sqlx::Database>::Connection #(, #fn_input)*) -> Result<(), sqlx::Error> where DB: sqlx::Database {
          sqlx::query!(#query #(, #query_params)*).execute(pool).await?;
          Ok(())
        }
      ).to_tokens(tokens);
    }
  }
}
