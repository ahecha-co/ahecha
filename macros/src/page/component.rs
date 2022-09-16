use std::fs::{read_dir, read_to_string};

use proc_macro2::TokenStream;
use quote::{quote, ToTokens, __private::Span};
use syn::Ident;

use super::{DynamicPageRoute, StaticPageRoute};
use crate::{base_module_path, Layout, TARGET_PATH};

#[derive(Clone, Debug)]
pub(crate) struct Component {
  pub(crate) children: Vec<Component>,
  pub(crate) ident: String,
  pub(crate) module_path: String,
  pub(crate) props: Vec<String>,
}

impl Component {
  pub fn build_recursive_up(page: Component) -> Component {
    let dir = read_dir(TARGET_PATH).unwrap();
    let mut layouts = vec![];
    let mut component_tree: Vec<Component> = vec![];

    for file_dir in dir {
      let file_dir = file_dir.unwrap();
      let path_str = file_dir.file_name().into_string().unwrap();
      if path_str.ends_with(".json") {
        let content = read_to_string(&file_dir.path()).unwrap();
        if path_str.starts_with("layout") {
          let value: Layout = serde_json::from_str(&content).unwrap();
          layouts.push(value);
        }
      }
    }

    let mut module_path_parts = {
      let base = base_module_path(&page.module_path);
      base.split("::").map(|s| s.to_owned()).collect::<Vec<_>>()
    };

    while !module_path_parts.is_empty() {
      let module_path = module_path_parts.join("::");

      if let Some(layout) = layouts
        .iter()
        .find(|l| base_module_path(&l.module_path) == module_path)
      {
        component_tree.push(layout.into());
      }

      let _ = module_path_parts.pop();
    }

    let mut component = page;

    for cmp_tree in component_tree.iter() {
      component = {
        let mut cmp_tree = cmp_tree.clone();
        cmp_tree.children.push(component);
        cmp_tree
      };
    }

    component
  }

  pub fn use_tokens(&self) -> TokenStream {
    let module_path = format!("{}::{}", &self.module_path, &self.ident)
      .parse::<TokenStream>()
      .unwrap();
    let mut tokens = vec![quote!( use #module_path ; )];
    for child in self.children.iter() {
      tokens.push(child.use_tokens());
    }
    quote!( #(#tokens)* )
  }
}

impl ToTokens for Component {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let ident = Ident::new(&self.ident, Span::call_site().into());
    let props = self
      .props
      .iter()
      .map(|ident| {
        let ident = ident.parse::<TokenStream>().unwrap();
        quote!( #ident: #ident .clone() )
      })
      .collect::<Vec<_>>();
    let children = &self.children;

    quote!(
      #ident {
        #(#props,)*
        #(#children)*
      }
    )
    .to_tokens(tokens);
  }
}

impl From<&DynamicPageRoute> for Component {
  fn from(item: &DynamicPageRoute) -> Self {
    Self {
      children: vec![],
      ident: item.ident.clone(),
      module_path: item.module_path.clone(),
      props: item.props.iter().map(|a| a.ident.clone()).collect(),
    }
  }
}

impl From<&StaticPageRoute> for Component {
  fn from(item: &StaticPageRoute) -> Self {
    Self {
      children: vec![],
      ident: item.ident.clone(),
      module_path: item.module_path.clone(),
      props: vec![],
    }
  }
}

impl From<&Layout> for Component {
  fn from(item: &Layout) -> Self {
    Self {
      children: vec![],
      ident: item.ident.clone(),
      module_path: item.module_path.clone(),
      props: vec![],
    }
  }
}
