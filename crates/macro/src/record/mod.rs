use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};

#[derive(Clone)]
struct Field {
  name: Ident,
  ty: Ident,
}

struct DeleteStatement {
  table: Table,
}

impl ToTokens for DeleteStatement {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    todo!()
  }
}

struct InsertStatement {
  table: Table,
}

impl ToTokens for InsertStatement {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    todo!()
  }
}

struct UpdateStatement {
  table: Table,
}

impl ToTokens for UpdateStatement {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    let mut fields = vec![];
    let mut set_statement = vec![];
    let mut where_statement = vec![];
    let mut fn_name_where_fields = vec![];
    let mut fn_input = vec![];
    let mut i = 0;

    self.table.fields.iter().for_each(|f| {
      let f_name = &f.name;
      fields.push(quote! { self. #f_name });
      set_statement.push(format!("{f_name} = ${i}"));
      i += 1;
    });

    self.table.where_fields.iter().for_each(|f| {
      let f_name = &f.name;
      let f_ty = &f.ty;
      fields.push(quote! { #f_name });
      where_statement.push(format!("{f_name} = ${i}"));
      fn_name_where_fields.push(f.name.to_string());
      fn_input.push(quote! { #f_name : #f_ty });
      i += 1;
    });

    let query = format!(
      "UPDATE {} SET {} WHERE {}",
      &self.table.name,
      set_statement.join(", "),
      where_statement.join(" AND ")
    );

    let fn_name = Ident::new(
      format!("update_where_{}", fn_name_where_fields.join("_and_")).as_str(),
      self.table.span,
    );

    quote! {
      fn async #fn_name <T>(&self, pool: &mut sqlx::PgPool, #(#fn_input),*) -> Result<T, sqlx::Error> {
        sqlx::query!(#query, #(#fields),*).fetch_one(&mut pool).await
      }
    }
    .to_tokens(tokens);
  }
}

#[derive(Clone, Copy, PartialEq)]
enum Statement {
  Delete,
  Insert,
  Update,
}

#[derive(Clone)]
pub struct Table {
  fields: Vec<Field>,
  name: String,
  returning_keys: Vec<Field>,
  span: Span,
  statement: Vec<Statement>,
  where_fields: Vec<Field>,
}

/*
```rust

#[derive(Table)]
#[insertable]
struct CreateUser {
  id: Uuid,
  first_name: String,
  last_name: String,
}

let user = CreateUser {...}
user.insert(&mut db_pool)?;

#[derive(Table)]
#[deletable(id: Uuid, tenant_id: String)]
struct DeleteUser;

DeleteUser::delete_where_id_and_tenant_id(&mut db_pool, id, tenant_id).await?;

#[derive(Table)]
#[deletable(id: Uuid, tenant_id: String)]
#[returning(id)]
struct UpdateUser {
  first_name: String,
  last_name: String,
}

let user = UpdateUser {...}
let id = user.update_where_id_and_tenant_id(&mut db_pool, id, tenant_id).await?;
```
*/

impl ToTokens for Table {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    if self.statement.contains(&Statement::Delete) {
      DeleteStatement {
        table: self.clone(),
      }
      .to_tokens(tokens);
    }

    if self.statement.contains(&Statement::Insert) {
      InsertStatement {
        table: self.clone(),
      }
      .to_tokens(tokens);
    }

    if self.statement.contains(&Statement::Update) {
      UpdateStatement {
        table: self.clone(),
      }
      .to_tokens(tokens);
    }
  }
}
