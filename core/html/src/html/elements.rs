pub enum HtmlElementType {
  CustomElement,
  Doctype,
  Page,
  Tag,
}

pub struct HtmlElement<A, C> {
  pub attributes: A,
  pub children: Option<C>,
  pub kind: HtmlElementType,
  pub name: &'static str,
}
