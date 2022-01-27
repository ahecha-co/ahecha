use proc_macro::Span;
use proc_macro2::{Ident, TokenStream};
use proc_macro_error::emit_error;
use quote::quote;
use syn::{DeriveInput, ItemFn};

pub enum HttpMethod {
  Delete,
  Get,
  Patch,
  Post,
  Put,
}

fn create_path() -> String {
  let span = Span::call_site();
  // Note: using source_file raises an error with rust analyzer. Ref: https://github.com/rust-analyzer/rust-analyzer/issues/10710#issuecomment-962559112
  let source = span.source_file();
  let path = source.path().to_str().unwrap().to_owned();
  let segments = path.split("src/").collect::<Vec<_>>();

  if segments.len() > 2 {
    emit_error!(span, "We don't support routes under an src user module, this means that you can't name your module `src`");
    format!("")
  } else if let Some(segment) = segments.last() {
    let url = segment
      .split("/")
      .into_iter()
      .filter_map(|v| {
        if v.ends_with(".rs") {
          let sufix = "_controller.rs";
          if v.ends_with(sufix) {
            Some(v[0..(v.len() - sufix.len())].to_owned())
          } else {
            emit_error!(span, "The naming convention dictates that this macro can only be used under a file named `{name}_controller.rs`");
            None
          }
        } else {
          Some(v.to_owned())
        }
      })
      .collect::<Vec<_>>()
      .join("/");

    format!("/{}", url)
  } else {
    emit_error!(span, "The file path isn't under an src/ folder");
    format!("")
  }
}

// the route macro will accept the following optional params:
// - literal string: relative path from the src/ and also using the controller name. example (auctions_controller.rs => "/auctions")
// - absolute_path: overrides the path with the provided one
//#[get(absolute_path = "/")]
pub fn create_route(_method: HttpMethod, item: ItemFn, _metadata: DeriveInput) -> TokenStream {
  let fn_name = Ident::new(
    format!("{}_path", item.sig.ident.to_string()).as_str(),
    item.sig.ident.span(),
  );
  let fn_route_name = Ident::new(
    format!("{}_route_path", item.sig.ident.to_string()).as_str(),
    item.sig.ident.span(),
  );
  // TODO: Extract params from url
  let use_args: Vec<TokenStream> = vec![];
  // TODO: Extract types from the fn arguments using use_args as ref
  let fn_args: Vec<TokenStream> = vec![];
  // TODO: Append the url path or replace with the one provided in the metadata (if any)
  let path = create_path();
  let route_path = create_path();

  quote!(
    #item

    pub fn #fn_name (#(#fn_args,)*) -> String {
      format!(#path #(, #use_args)*)
    }

    pub fn #fn_route_name () -> &'static str {
      #route_path
    }
  )
}
