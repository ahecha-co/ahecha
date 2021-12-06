use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn create_model(_input: DeriveInput) -> TokenStream {
  quote!()
  // let struct_name = s.clone().ident;
  // let struct_metadata = Ident::new(
  //   format!("__{}_metadata", struct_name).as_str(),
  //   struct_name.span(),
  // );
  // let ModelAttributes { table_name, pk } = ModelAttributes::from_meta(&attrs);
  // let fields = if let Fields::Named(fields) = s.clone().fields {
  //   fields
  // } else {
  //   abort!(s.fields, "Only named fields struct are supported");
  // }
  // .named
  // .iter()
  // .map(|f| f.clone())
  // .collect::<Vec<_>>();
  // let pk_ty = "";

  // let delete_query = format!("DELETE FROM {} WHERE {} = $1;", table_name, pk);

  // let insert_query = format!(
  //   "INSERT INTO {} ({}) VALUES ({});",
  //   table_name,
  //   fields
  //     .iter()
  //     .map(|f| f.ident.as_ref().unwrap().to_string())
  //     .collect::<Vec<_>>()
  //     .join(","),
  //   fields
  //     .iter()
  //     .enumerate()
  //     .map(|(i, _)| (i + 1).to_string())
  //     .collect::<Vec<_>>()
  //     .join(",")
  // );

  // let find_query = format!(
  //   "SELECT {} FROM {} WHERE {}=$1",
  //   fields
  //     .iter()
  //     .map(|f| f.ident.as_ref().unwrap().to_string())
  //     .collect::<Vec<_>>()
  //     .join(","),
  //   table_name,
  //   pk,
  // );

  // let find_by_query = format!(
  //   "SELECT {} FROM {} WHERE {{}}",
  //   fields
  //     .iter()
  //     .map(|f| f.ident.as_ref().unwrap().to_string())
  //     .collect::<Vec<_>>()
  //     .join(","),
  //   table_name,
  // );

  // let update_query = format!(
  //   "UPDATE {} SET {} WHERE {}=${};",
  //   table_name,
  //   fields
  //     .iter()
  //     .enumerate()
  //     .map(|(i, f)| format!("{}=${}", f.ident.as_ref().unwrap(), i + 1))
  //     .collect::<Vec<_>>()
  //     .join(","),
  //   pk,
  //   fields.len() + 1
  // );

  // let field_variants = fields
  //   .iter()
  //   .map(|f| {
  //     let ident = f.ident.as_ref().unwrap();
  //     let ty = f.ty.clone();
  //     quote!( #ident ( #ty ))
  //   })
  //   .collect::<Vec<_>>();
  // let field_match_arms = fields
  //   .iter()
  //   .map(|f| {
  //     let ident = f.ident.as_ref().unwrap();
  //     quote!( #ident (v) => v.to_string())
  //   })
  //   .collect::<Vec<_>>();

  // quote!(
  //   pub mod #struct_metadata {
  //     pub enum Field {
  //       #(#field_variants,)*
  //     }

  //     impl ToString for Field {
  //       fn to_string(&self) -> String {
  //         let (field, value) = match self {
  //           #(#field_match_arms,)*
  //         };

  //         format!("{}={}", field, value)
  //       }
  //     }
  //   }

  //   impl ahecha::Model for #struct_name {
  //     await fn create(&self, executor: sqlx::PgPool) -> Result<(), sqlx::Error> {
  //       let Self { #(#fields,)* } = self;
  //       sqlx::query_as!(Self, #insert_query, #(#fields,)* ).execute(executor).await
  //     }

  //     await fn delete(&self, executor: sqlx::PgPool) -> Result<(), sqlx::Error> {
  //       sqlx::query!(#delete_query, self. #pk).execute(executor).await
  //     }

  //     await fn find(executor: sqlx::PgPool, #pk: #pk_ty) -> Result<Self, sqlx::Error> {
  //       sqlx::query_as!(Self, #find_query, #pk).fetch_one(executor).await
  //     }

  //     await fn find_by(
  //       executor: sqlx::PgPool,
  //       field: #struct_metadata ::Field,
  //     ) -> Result<Vec<Self>, sqlx::Error> {
  //       sqlx::query_as(Self, format!(#find_by_query, field.to_string()).as_str()).fetch_one(executor).await
  //     }

  //     await fn update(&self, executor: sqlx::PgPool) -> Result<(), sqlx::Error> {
  //       let Self { #(#fields),* } = self;
  //       sqlx::query_as!(Self, #update_query, #(#fields,)*, self. #pk ).execute(executor).await
  //     }
  //   }
  // )
}
