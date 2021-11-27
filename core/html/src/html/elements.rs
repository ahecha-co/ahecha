pub struct HtmlElement<A, C> {
  pub name: &'static str,
  pub attributes: A,
  pub children: Option<C>,
}
