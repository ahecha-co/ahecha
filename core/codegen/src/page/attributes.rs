use proc_macro2::Span;
use syn::{LitStr, NestedMeta, Path};

pub struct PageAttributes {
  pub document: Path,
  pub layout: Option<Path>,
}

impl PageAttributes {
  pub fn from_meta(meta: &Vec<NestedMeta>) -> Self {
    let mut document = None;
    let mut layout = None;

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
