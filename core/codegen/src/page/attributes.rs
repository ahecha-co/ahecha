use proc_macro2::Span;
use syn::{LitStr, NestedMeta, Path};

pub struct PageAttributes {
  pub document: Path,
  pub layout: Option<Path>,
  pub title: Option<String>,
}

impl PageAttributes {
  pub fn from_meta(meta: &Vec<NestedMeta>) -> Self {
    let mut document = None;
    let mut layout = None;
    let mut title = None;

    for meta in meta {
      match meta {
        NestedMeta::Meta(meta) => match meta {
          syn::Meta::NameValue(meta) => match meta.path.get_ident().unwrap().to_string().as_str() {
            "document" => {
              document = lit_to_path(&meta.lit);
            }
            "layout" => {
              layout = lit_to_path(&meta.lit);
            }
            "title" => match &meta.lit {
              syn::Lit::Str(lit) => title = Some(lit.value().to_string()),
              _ => panic!("The title of the page must be a string literal"),
            },
            _ => {}
          },
          _ => {}
        },
        _ => {}
      }
    }

    Self {
      document: if let Some(document) = document {
        document
      } else {
        LitStr::new("crate::document::Document", Span::call_site())
          .parse()
          .unwrap()
      },
      layout,
      title,
    }
  }
}

fn lit_to_path(lit: &syn::Lit) -> Option<Path> {
  match lit {
    syn::Lit::Str(lit) => match lit.parse() {
      Ok(path) => Some(path),
      Err(_) => None,
    },
    _ => None,
  }
}
