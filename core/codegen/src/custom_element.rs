use proc_macro::TokenStream;
// use proc_macro2::Span;
use proc_macro_error::emit_error;
use quote::quote;
// use syn::Pat;

use crate::utils::FnStruct;

pub fn create_custom_element(f: syn::ItemFn) -> TokenStream {
  let fn_struct: FnStruct = f.into();

  let vis = fn_struct.vis();
  let struct_name = fn_struct.name();
  // let mut observed_attributes = vec![];
  // let mut update_attribute_values = vec![];

  // for attr in fn_struct.input_names().iter() {
  //   match attr {
  //     Pat::Ident(ident) => {
  //       let attr_name = ident.ident.to_string();
  //       observed_attributes.push(quote!(#attr_name));
  //       update_attribute_values.push(quote!(
  //         #attr_name => self. #attr = if let Some(new_value) = new_value {
  //           ahecha::serde_json::from_str(new_value.as_str())
  //           //.expect(format!("Could not deserialize the value `{}`", new_value))
  //           .unwrap()
  //         } else {
  //           Default::default()
  //         },
  //       ));
  //     }
  //     _ => emit_error!(Span::call_site(), "Attribute names must be identifiers",),
  //   };
  // }

  let struct_str_name = struct_name.to_string();
  if !fn_struct.has_camel_case_name("Custom elements must have a name") {
    emit_error!(
      struct_name.span(),
      "Custom elements must start with a upper letter"
    );
  }

  if struct_str_name.ends_with("Partial") || struct_str_name.ends_with("Page") {
    emit_error!(
      struct_name.span(),
      "Custom elements cannot end with `Partial` or `Page` suffix.",
    );
  }

  let view_fn = fn_struct.create_view();

  quote! {
    #[allow(non_snake_case)]
    #vis mod #struct_name {
      use super::*;

      // #[derive(Debug, Default)]
      // #[cfg(feature = "frontend")]
      // #vis struct #struct_name #impl_generics {}
        // Implement some struct here to handle the component state maybe?
        // state: State, ??
        // event_listeners: Vec<EventListener>,??
        // el: HtmlElement,??
      // }

      // #[cfg(feature = "frontend")]
      // impl #impl_generics #struct_name #ty_generics #where_clause {
      //   // pub fn define() {
      //   //   gloo_utils::window().custom_elements().define(
      //   //     #struct_str_name,
      //   //     #struct_name::new,
      //   //   );
      //   // }

      //   pub fn register() {
      //     // use ahecha::view::CustomElement;
      //     // Self::define(#struct_str_name);
      //   }
      // }

      // #[cfg(feature = "frontend")]
      // impl #impl_generics ahecha::view::CustomElement for #struct_name #ty_generics #where_clause {
      //   // #[wasm_bindgen(constructor)]
      //   // fn constructor(&mut self) {
      //   //   let (style, template) = self.get_template();
      //   //   let document = gloo_utils::document();
      //   //   // TODO: Eventually support and extract the style tag from the template
      //   //   // let style_tag = document.create_element("style").unwrap_throw();
      //   //   // style_tag.set_inner_html(style);

      //   //   match el.shadow_root() {
      //   //     Some(shadow_root) => {
      //   //       // shadow_root.append_child(&style_tag).unwrap_throw()
      //   //       shadow_root.append_child(&template).unwrap_throw()
      //   //     }
      //   //     None => {
      //   //       // el.append_child(&style_tag).unwrap_throw();
      //   //       el.append_child(&template).unwrap_throw()
      //   //     }
      //   //   }
      //   // }

      //   // fn get_template(&self) -> (web_sys::HtmlStyleElement, web_sys::HtmlElement) {
      //   //   (,)
      //   // }

      //   fn inject_children(&mut self, this: &web_sys::HtmlElement) {
      //     // inject_style(&this, "p { color: green; }");
      //     let node: String = self.render();
      //     this.set_inner_text(&node.as_str());
      //   }

      //   fn observed_attributes() -> &'static [&'static str] {
      //     &[#(#observed_attributes),*]
      //   }
      //   fn attribute_changed_callback(
      //     &mut self,
      //     _this: &web_sys::HtmlElement,
      //     name: String,
      //     _old_value: Option<String>,
      //     new_value: Option<String>,
      //   ) {
      //     match name.as_str() {
      //       #(#update_attribute_values)*
      //       _ => {}
      //     }
      //   }

      //   fn connected_callback(&mut self, _this: &web_sys::HtmlElement) {
      //     // log("connected");
      //   }

      //   fn disconnected_callback(&mut self, _this: &web_sys::HtmlElement) {
      //     // log("disconnected");
      //   }

      //   fn adopted_callback(&mut self, _this: &web_sys::HtmlElement) {
      //     // log("adopted");
      //   }
      // }

      // #[cfg(feature = "frontend")]
      // impl #impl_generics ahecha::view::RenderNode for #struct_name #ty_generics #where_clause {
      //   fn render(&self) -> web_sys::Node {
      //     return {
      //       #input_readings
      //       #block
      //     }.render()
      //   }
      // }

      #view_fn
    }
  }
  .into()
}
