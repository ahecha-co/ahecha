use proc_macro2::TokenStream;
use proc_macro_error::emit_error;
use quote::quote;
use syn::{parse::Parse, Data, DeriveInput, Field, Type, TypePath};

struct Table {
  name: String,
  fields: Vec<Field>,
}

impl Parse for Table {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let derive = input.parse::<DeriveInput>()?;
    let mut name = derive.ident.clone().to_string().to_lowercase();
    let fields = match derive.data {
      Data::Struct(data) => vec![],
      _ => {
        emit_error!(
          derive.ident.span(),
          "The `Table` derive can be used only on `structs`"
        );
        vec![]
      }
    };

    if let Some(table_name) = derive
      .attrs
      .iter()
      .filter_map(|a| {
        if let Some(segment) = a.path.segments.last() {
          if segment.ident.to_string() == "Table" {
            todo!("Implement this");
            Some("----------".to_string())
          } else {
            None
          }
        } else {
          None
        }
      })
      .collect::<Vec<_>>()
      .first()
    {
      todo!("Extract table name if is set");
    } else {
      name = inflection::plural(name);
    }

    Ok(Self { name, fields })
  }
}

pub fn create_table(_input: TokenStream) -> TokenStream {
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

pub fn create_queryable(item: DeriveInput) -> TokenStream {
  let name = &item.ident;
  let columns = match item.data.clone() {
    Data::Enum(_) => {
      emit_error!(
        &item.ident.span(),
        "Cannot implement over an enum, only struct is supported"
      );
      vec![]
    }
    Data::Struct(data) => data
      .fields
      .into_iter()
      .filter_map(|f| {
        let name = f
          .ident
          .clone()
          .expect("Field to have identifier")
          .to_string();

        if name.starts_with("__") {
          None
        } else {
          let span = f.ident.clone().expect("Field to have identifier").span();
          // TODO: implement this propertly via some derive attribute
          let is_primary_key = name == "id";
          let sqlx_cast_as_underscore = match &f.ty {
            Type::BareFn(_)
            | Type::Group(_)
            | Type::ImplTrait(_)
            | Type::Macro(_)
            | Type::Never(_)
            | Type::Paren(_) => {
              emit_error!(span, "Unsuported type");
              true
            }
            Type::Array(_) => true,
            Type::Infer(_) => true,
            Type::Path(TypePath { path, .. }) => {
              let segment = path.segments.last().unwrap().ident.clone().to_string();
              !vec![
                "f32",
                "f64",
                "i8",
                "i16",
                "i32",
                "i64",
                "i128",
                "isize",
                "u8",
                "u16",
                "u32",
                "u64",
                "u128",
                "usize",
                "String",
                "&str",
                "&'static str",
                "bool",
                // Custom know types
                "Uuid",
              ]
              .contains(&segment.as_str())
            }
            Type::Ptr(_) => true,
            Type::Reference(_) => true,
            Type::Slice(_) => true,
            Type::TraitObject(_) => true,
            Type::Tuple(_) => true,
            Type::Verbatim(_) => true,
            Type::__TestExhaustive(_) => todo!(),
          };
          Some(quote! {
            TableColumn {
              name: #name,
              is_primary_key: #is_primary_key,
              sqlx_cast_as_underscore: #sqlx_cast_as_underscore,
            }
          })
        }
      })
      .collect::<Vec<_>>(),
    Data::Union(_) => {
      emit_error!(
        &item.ident.span(),
        "Cannot implement over a union, only struct is supported"
      );
      vec![]
    }
  };
  quote! {
    impl ahecha::database::Queryable for #name {
      fn columns() -> Vec<ahecha::database::TableColumn> {
        vec![#(#columns),*]
      }
    }
  }
}
