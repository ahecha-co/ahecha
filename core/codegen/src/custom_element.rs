use proc_macro::TokenStream;
use proc_macro_error::emit_error;
use quote::quote;

use crate::utils::FnStruct;

pub fn create_custom_element(f: syn::ItemFn) -> TokenStream {
  let fn_struct: FnStruct = f.into();

  let vis = fn_struct.vis();
  let struct_name = fn_struct.name();
  let block = fn_struct.block();
  let impl_generics = fn_struct.impl_generics();
  let ty_generics = fn_struct.type_generics();
  let where_clause = fn_struct.where_clause();
  let input_blocks = fn_struct.input_blocks();
  let input_fields = fn_struct.inputs();
  let mut observed_attributes = vec![];
  let mut update_attribute_values = vec![];

  for attr in input_fields.iter() {
    let attr_name = stringify!(#_f);

    observed_attributes.push(quote!(#attr_name));
    update_attribute_values.push(quote!(
      #attr_name => self. #attr = if let Some(new_value) = new_value {
        new_value.into()
      } else {
        Default::default()
      },
    ));
  }

  let input_readings = fn_struct.input_readings();

  let struct_str_name = struct_name.to_string();
  if struct_str_name.to_uppercase().chars().next().unwrap()
    != struct_str_name.chars().next().unwrap()
  {
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

  quote! {
    #[cfg(feature = "backend")]
    #[derive(Debug, Default)]
    #vis struct #struct_name #impl_generics #input_blocks

    #[cfg(feature = "frontend")]
    #[derive(Debug, Default)]
    #vis struct #struct_name #impl_generics {}
      // Implement some struct here to handle the component state maybe?
      // state: State, ??
      // event_listeners: Vec<EventListener>,??
      // el: HtmlElement,??
    // }

    #[cfg(feature = "frontend")]
    impl #impl_generics #struct_name #ty_generics #where_clause {
      // pub fn define() {
      //   gloo_utils::window().custom_elements().define(
      //     #struct_str_name,
      //     #struct_name::new,
      //   );
      // }

      pub fn register() {
        use custom_elements::CustomElement;
        Self::define(#struct_str_name);
      }
    }

    #[cfg(feature = "frontend")]
    impl #impl_generics custom_elements::CustomElement for #struct_name #ty_generics #where_clause {
      // #[wasm_bindgen(constructor)]
      // fn constructor(&mut self) {
      //   let (style, template) = self.get_template();
      //   let document = gloo_utils::document();
      //   // TODO: Eventually support and extract the style tag from the template
      //   // let style_tag = document.create_element("style").unwrap_throw();
      //   // style_tag.set_inner_html(style);

      //   match el.shadow_root() {
      //     Some(shadow_root) => {
      //       // shadow_root.append_child(&style_tag).unwrap_throw()
      //       shadow_root.append_child(&template).unwrap_throw()
      //     }
      //     None => {
      //       // el.append_child(&style_tag).unwrap_throw();
      //       el.append_child(&template).unwrap_throw()
      //     }
      //   }
      // }

      // fn get_template(&self) -> (web_sys::HtmlStyleElement, web_sys::HtmlElement) {
      //   (,)
      // }

      fn inject_children(&mut self, this: &web_sys::HtmlElement) {
        // inject_style(&this, "p { color: green; }");
        let node: String = self.render();
        this.set_inner_text(&node.as_str());
      }

      fn observed_attributes() -> &'static [&'static str] {
        &[#(#observed_attributes),*]
      }
      fn attribute_changed_callback(
        &mut self,
        _this: &web_sys::HtmlElement,
        name: String,
        _old_value: Option<String>,
        new_value: Option<String>,
      ) {
        match name {
          #(#update_attribute_values)*
          _ => {}
        }
      }

      fn connected_callback(&mut self, _this: &web_sys::HtmlElement) {
        // log("connected");
      }

      fn disconnected_callback(&mut self, _this: &web_sys::HtmlElement) {
        // log("disconnected");
      }

      fn adopted_callback(&mut self, _this: &web_sys::HtmlElement) {
        // log("adopted");
      }
    }

    #[cfg(feature = "frontend")]
    impl #impl_generics ahecha::view::RenderNode for #struct_name #ty_generics #where_clause {
      fn render(&self) -> web_sys::Node {
        return {
          #input_readings
          #block
        }.render()
      }
    }

    impl #impl_generics ahecha::view::RenderString for #struct_name #ty_generics #where_clause {
      fn render_into<W: std::fmt::Write>(self, w: &mut W) -> ::std::fmt::Result {
        let result = {
          #input_readings
          #block
        }.render();

        write!(w, "{}", result)
      }
    }

    impl #impl_generics Into<String> for #struct_name #ty_generics #where_clause {
      fn into(self) -> String {
        use ahecha::view::RenderString;
        let mut result = String::new();
        self.render_into(&mut result).unwrap();
        result
      }
    }
  }
  .into()
}
