use crate::Html;

// Define a macro attribute to create an element
//
// ```rust
// #[component('component-name' => HTMLElement)]
// struct MyComponent {}
// ```
//
// The component name should have always a hyphen (`-`), if no element if given it will extend from
// HTMLElement, this also will register the component to be used in the html macro.
//
// Components need to be able to be rendered in server side too.
pub trait Component: Default {
  /// An instance of the element is created or upgraded. Useful for initializing state, setting up event listeners, or creating a shadow dom. See the spec for restrictions on what you can do in the constructor.
  fn new() -> Self {
    Self::default()
  }
  /// Called every time the element is inserted into the DOM. Useful for running setup code, such as fetching resources or rendering. Generally, you should try to delay work until this time.
  fn connected_callback(&self) {}
  /// Called every time the element is removed from the DOM. Useful for running clean up code.
  fn disconnected_callback(&self) {}
  /// Called when an observed attribute has been added, removed, updated, or replaced. Also called for initial values when an element is created by the parser, or upgraded. Note: only attributes listed in the observed_attributes property will receive this callback.
  fn attribute_change_callback() {}
  /// The custom element has been moved into a new document (e.g. someone called document.adopt_node(el)).
  // fn adopted_callback(&self, el: HTMLElement) {}
  /// Renders the component
  fn render(&self) -> Html;
}
