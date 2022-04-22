use crate::page::Component;

pub struct Route {
  path: String,
  // view: Body,
}

pub struct RouterComponent {
  routes: Vec<Route>,
}

impl Component for RouterComponent {
  fn render(&self, _scope: crate::view::Scope) -> ahecha_html::Node {
    todo!()
  }
}
