use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};

use super::WhereConstraint;

pub struct DeleteStatement {
  pub span: Span,
  pub table_name: String,
  pub constraint: WhereConstraint,
}

impl ToTokens for DeleteStatement {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let (where_query, where_params) = self.constraint.build(0);
    let fields = self
      .constraint
      .fields
      .iter()
      .map(|f| f.name.to_string())
      .collect::<Vec<_>>();
    let fn_input = &self.constraint.fields;

    let fn_ident = if !fields.is_empty() {
      Ident::new(
        format!("delete_where_{}", fields.join("_and_")).as_str(),
        self.span,
      )
    } else {
      Ident::new("delete", self.span)
    };

    let query = format!("DELETE FROM {} {}", &self.table_name, where_query);
    quote!(
      pub async fn #fn_ident <DB> (&self, mut pool: <sqlx::Postgres as sqlx::Database>::QueryResult #(, #fn_input)*) -> Result<(), sqlx::Error> where DB: sqlx::Database {
        sqlx::query!(#query #(, #where_params)*).execute(&mut pool).await?;
        Ok(())
      }
    )
    .to_tokens(tokens);
  }
}
