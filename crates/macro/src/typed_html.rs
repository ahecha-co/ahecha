use proc_macro2::{Ident, Span};
use syn::parse::Parse;

struct Attr {
  name: Ident,
  ty: Ident,
}

impl Attr {
  fn new(span: Span, name: &str, ty: &str) -> Self {
    Self {
      name: Ident::new(name, span),
      ty: Ident::new(ty, span),
    }
  }

  fn global(span: Span) -> Vec<Attr> {
    vec![
      Attr::new(span, "accesskey", "Option<String>"),
      Attr::new(span, "class", "Option<Vec<String>>"),
      Attr::new(span, "contenteditable", "Option<BoolInherit>"), // https://www.w3.org/TR/2011/WD-html5-20110405/editing.html#attr-contenteditable
      Attr::new(span, "contextmenu", "Option<String>"),
      Attr::new(span, "dir", "Option<Directionality>"), // https://www.w3.org/TR/2011/WD-html5-20110405/elements.html#the-dir-attribute
      Attr::new(span, "draggable", "Option<bool>"),
      Attr::new(span, "dropzone", "Option<Dropzone>"), // https://www.w3.org/TR/2011/WD-html5-20110405/dnd.html#the-dropzone-attribute
      Attr::new(span, "hidden", "Option<bool>"),
      Attr::new(span, "id", "Option<String>"),
      Attr::new(span, "lang", "Option<String>"),
      Attr::new(span, "spellcheck", "Option<bool>"),
      Attr::new(span, "style", "Option<CSSStyleDeclaration>"), // https://www.w3.org/TR/2011/WD-html5-20110405/elements.html#the-style-attribute
      Attr::new(span, "tabindex", "Option<isize>"),
      Attr::new(span, "title", "Option<String>"),
    ]
  }
}

struct Tag {
  name: Ident,
  attrs: Vec<Attr>,
}

struct ElementGroup {
  name: Ident,
  tags: Vec<Tag>,
}

/*
html {
  xmlns: Uri,
} << [head, body];
head << [MetadataContent];
body << [FlowContent];

// Metadata
base {
  href: Uri,
  target: Target,
} in [MetadataContent];
*/
pub struct TypedHtml {
  groups: Vec<ElementGroup>,
}

impl TypedHtml {
  fn push(&mut self, _ident: Ident, _attrs: Vec<Attr>) {
    todo!("Implement TypedHtml::push")
  }
}

impl Parse for TypedHtml {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let mut typed_html = TypedHtml { groups: vec![] };

    while !input.is_empty() {
      let ident = input.parse::<Ident>()?;
      let attrs: Vec<Attr> = Attr::global(ident.span());

      if input.peek(syn::token::Brace) {
        // TODO: parse attrs
      }

      if input.peek(syn::token::In) {
        // TODO: parse group array
      }

      if input.peek(syn::token::Shl) {
        let ident = input.parse::<Ident>()?;
        if ident == "with" {
          // TODO: parse children groups
        }
      }

      input.parse::<syn::token::Semi>()?;

      typed_html.push(ident, attrs);
    }

    Ok(typed_html)
  }
}
