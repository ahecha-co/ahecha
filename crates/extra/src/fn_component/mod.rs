use std::{fmt::Display, rc::Rc};

use serde::{de::DeserializeOwned, Serialize};

// TODO: This trait will enable the state to tell the context that it has been updated
pub trait ContextState: Serialize {}

pub struct Context<S = ()>
where
  S: ContextState,
{
  pub state: S,
  scope_id: usize,
  // TODO: hide behind `frontend` feature flag
  document: Option<web_sys::Document>,
}

impl<S> Context<S>
where
  S: ContextState,
{
  pub fn new() -> Self {
    Self::from_state(())
  }

  pub fn from_state(state: S) -> Self {
    Self {
      state,
      scope_id: 1,
      document: None,
    }
  }

  // TODO: hide behind `frontend` feature flag
  pub fn from_document(document: web_sys::Document) -> Self {
    Self {
      // TODO: hydrate the context state, maybe write a script tag with the json state and read from there.
      state: (),
      scope_id: 1,
      document: Some(document),
    }
  }

  // TODO: add implementation for when the `frontend` feature flag is enabled and other without
  fn scope<P>(&mut self) -> Scope<P, S>
  where
    P: Props,
  {
    if let Some(document) = self.document.as_ref() {
      // TODO: get the scope params from the tag `data-props-*` attrs
      self.scope_with_props(())
    } else {
      self.scope_with_props(())
    }
  }

  fn scope_with_props<P>(&mut self, props: P) -> Scope<P, S>
  where
    P: Props,
  {
    let id = self.scope_id;
    self.scope_id += 1;
    Scope::new(id, Rc::new(&self), props)
  }
}

impl Display for Context {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "<script>const CONTEXT_STATE_DATA = {};</script>",
      serde_json::to_string(&self.state)
    )
  }
}

pub struct Scope<P = (), S = ()>
where
  S: ContextState,
  P: Props,
{
  id: String,
  scope_id: usize,
  context: Rc<Context<S>>,
  props: P,
}

impl<P, S> Scope<P, S>
where
  S: ContextState,
  P: Props,
{
  pub fn new(id: usize, context: Rc<Context<S>>, props: P) -> Self {
    Self {
      id: id.to_string(),
      scope_id: 0,
      context,
      props,
    }
  }

  fn child_scope<PP>(&mut self) -> Self
  where
    PP: Props,
  {
    if let Some(document) = self.context.document.as_ref() {
      // TODO: hydrate
      self.child_scope_with_props(())
    } else {
      self.child_scope_with_props(())
    }
  }

  fn child_scope_with_scope<PP>(&mut self, props: PP) -> Self
  where
    PP: Props,
  {
    let id = self.scope_id;
    self.scope_id += 1;
    Self {
      id: format!("{}.{}", self.id, id),
      scope_id: 0,
      context: self.context.clone(),
      props,
    }
  }

  fn hydration_id(&self) -> HydrationID {
    HydrationID {
      id: if self.context.hydrate() {
        Some(self.id.clone())
      } else {
        None
      },
    }
  }

  fn get_props(document: &web_sys::Document, id: &str) -> P
  where
    P: Props,
  {
    // TODO: deal with the error and if element isn't found
    let el = document
      .query_selector(format!(r#"[data-hk="{id}"]"#).as_str())
      .unwrap()
      .unwrap();
    let mut attrs = serde_json::Map::new();
    let names: Vec<wasm_bindgen::JsValue> = el.get_attribute_names().to_vec();
    names
      .iter()
      .filter_map(|key| key.as_string())
      .filter(|key| key.as_str().starts_with("data-props-"))
      .for_each(|key| {
        attrs.insert(
          key.clone(),
          match el.get_attribute(&key) {
            Some(value) => serde_json::Value::String(value),
            None => serde_json::Value::Null,
          },
        );
      });
    let json = serde_json::Value::Object(attrs);
    // TODO: deal with the error
    serde_json::from_value(json).unwrap()
  }
}

#[derive(Default)]
pub struct SerializedProps {
  props: Vec<(String, Option<String>)>,
}

impl Display for SerializedProps {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.props.iter().try_for_each(|(key, value)| match value {
      Some(value) => write!(f, r#" data-props-{}="{}""#, key, value),
      None => write!(f, " {}", key),
    })
  }
}

pub trait Props: DeserializeOwned {
  fn serialize_props(&self) -> SerializedProps;
}

impl Props for () {
  fn serialize_props(&self) -> SerializedProps {
    SerializedProps { props: vec![] }
  }
}

struct HydrationID {
  id: Option<String>,
}

impl Display for HydrationID {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.id.as_ref() {
      Some(id) => write!(f, r#" data-hk="{}""#, id),
      None => Ok(()),
    }
  }
}

pub trait Context: Default {
  fn hydrate(&self) -> bool {
    true
  }
}

impl Context for () {}

pub struct Element {
  name: String,
  props: SerializedProps,
  children: Node,
  hydration_id: HydrationID,
}

impl Display for Element {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "<{}{}{}>{}</{}>",
      &self.name, &self.hydration_id, &self.props, &self.children, &self.name
    )
  }
}

pub struct CustomElement {
  name: String,
  props: SerializedProps,
  children: Node,
  hydration_id: HydrationID,
}

impl Display for CustomElement {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "<{}{}{}>{}</{}>",
      &self.name, &self.hydration_id, &self.props, &self.children, &self.name
    )
  }
}

pub enum Node {
  CustomElement(Box<CustomElement>),
  Element(Box<Element>),
  Fragment(Vec<Node>),
  None,
  Text(String),
}

impl Node {
  fn bind(&self, document: web_sys::Document) {}
}

impl Display for Node {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      // Node::Component(cmp) => match cmp(Scope { context: &() }) {
      //   Some(node) => write!(f, "{}", node),
      //   None => Ok(()),
      // },
      // Node::CustomElement(name, cmp) => {
      //   write!(f, "<{}>", &name);
      //   match cmp(Scope { context: &() }) {
      //     Some(node) => write!(f, "{}", node)?,
      //     None => {}
      //   }
      //   write!(f, "</{}>", &name)
      // }
      Node::CustomElement(element) => write!(f, "{}", element),
      Node::Element(element) => write!(f, "{}", element),
      Node::Fragment(nodes) => nodes.iter().try_for_each(|node| write!(f, "{}", node)),
      Node::None => Ok(()),
      Node::Text(text) => write!(f, "{}", text),
    }
  }
}

pub struct Document {
  body: Node,
}

impl Display for Document {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "<html><body>{}</body></html>", self.body)
  }
}

pub mod rendering {
  use super::*;

  pub mod client {
    use serde::de::DeserializeOwned;

    use super::*;

    pub struct Options {
      entry_point: String,
    }

    impl Default for Options {
      fn default() -> Self {
        Self {
          entry_point: "body".to_string(),
        }
      }
    }

    pub fn hydrate<'de, P, C>(component: Component<P, C>, context: C, _options: Options)
    where
      P: Props + DeserializeOwned,
      C: Context,
    {
      let document = web_sys::window().unwrap().document().unwrap();
      // let entry_point = document.query_selector(&options.entry_point).unwrap().unwrap();
      component(Scope::from_document(document.clone(), "1", context)).bind(document);
      // TODO: parse entry point to hydrate the node
      // TODO: mount the hydrated node
    }
  }

  pub mod server {
    use super::*;

    pub fn render(doc: Document) -> String {
      format!("{}", doc)
    }
  }
}

type Component<P = (), C = ()> = fn(cx: Scope<P, C>) -> Node;

#[cfg(test)]
mod test {
  use serde::Deserialize;

  use super::*;

  #[test]
  fn test_empty_fn_component() {
    fn test_component(_: Scope) -> Node {
      Node::None
    }

    let doc = Document {
      body: test_component(Scope::new("1", Rc::new(()), ())),
    };

    assert_eq!("<html><body></body></html>", rendering::server::render(doc));
  }

  #[test]
  fn test_fn_component() {
    fn test_component(_: Scope) -> Node {
      Node::Text("Test".to_owned())
    }

    let doc = Document {
      body: test_component(Scope::new("1", Rc::new(()), ())),
    };

    assert_eq!(
      "<html><body>Test</body></html>",
      rendering::server::render(doc)
    );
  }

  #[test]
  fn test_nested_fn_component() {
    fn test_component(cx: Scope) -> Node {
      test_nested_component(Scope::with(&cx, "1", ()))
    }

    fn test_nested_component(_: Scope) -> Node {
      Node::Text("Nested".to_owned())
    }

    let doc = Document {
      body: test_component(Scope::new("1", Rc::new(()), ())),
    };

    assert_eq!(
      "<html><body>Nested</body></html>",
      rendering::server::render(doc)
    );
  }

  #[test]
  fn test_custom_element() {
    fn test_component(cx: Scope) -> Node {
      fn inner_fn(cx: Scope) -> Node {
        test_nested_component(Scope::with(&cx, "1", ()))
      }

      Node::CustomElement(Box::new(CustomElement {
        name: "custom-element".to_owned(),
        props: cx.props.serialize_props(),
        hydration_id: cx.hydration_id(),
        children: inner_fn(cx),
      }))
    }

    fn test_nested_component(_: Scope) -> Node {
      Node::Text("Nested".to_owned())
    }

    let doc = Document {
      body: test_component(Scope::new("1", Rc::new(()), ())),
    };

    assert_eq!(
      r#"<html><body><custom-element data-hk="1">Nested</custom-element></body></html>"#,
      doc.to_string()
    );
  }

  #[test]
  fn test_fn_component_with_props() {
    #[derive(Default)]
    struct TestContext;

    impl Context for TestContext {
      fn hydrate(&self) -> bool {
        false
      }
    }

    #[derive(Deserialize)]
    struct TestProps {
      id: i32,
    }

    impl Props for TestProps {
      fn serialize_props(&self) -> SerializedProps {
        SerializedProps {
          props: vec![("id".to_string(), Some(self.id.to_string()))],
        }
      }
    }

    fn test_component(cx: Scope<TestProps, TestContext>) -> Node {
      Node::Element(Box::new(Element {
        name: "div".to_owned(),
        props: Default::default(),
        hydration_id: Scope::with(&cx, "1", ()).hydration_id(),
        children: Node::Text(cx.props.id.to_string()),
      }))
    }

    let doc = Document {
      body: test_component(Scope::new("1", Rc::new(TestContext), TestProps { id: 1 })),
    };

    assert_eq!(
      "<html><body><div>1</div></body></html>",
      rendering::server::render(doc)
    );
  }

  #[test]
  fn test_custom_element_with_props() {
    #[derive(Default)]
    struct TestContext;

    impl Context for TestContext {
      fn hydrate(&self) -> bool {
        false
      }
    }

    #[derive(Deserialize)]
    struct TestProps {
      id: i32,
    }

    impl Props for TestProps {
      fn serialize_props(&self) -> SerializedProps {
        SerializedProps {
          props: vec![("id".to_string(), Some(self.id.to_string()))],
        }
      }
    }

    fn test_component(cx: Scope<TestProps, TestContext>) -> Node {
      fn inner_fn(cx: Scope<TestProps, TestContext>) -> Node {
        Node::Element(Box::new(Element {
          name: "div".to_owned(),
          props: Default::default(),
          hydration_id: Scope::with(&cx, "1", ()).hydration_id(),
          children: Node::Text(cx.props.id.to_string()),
        }))
      }

      Node::CustomElement(Box::new(CustomElement {
        name: "custom-element".to_owned(),
        props: cx.props.serialize_props(),
        hydration_id: cx.hydration_id(),
        children: inner_fn(cx),
      }))
    }

    let doc = Document {
      body: test_component(Scope::new("1", Rc::new(TestContext), TestProps { id: 1 })),
    };

    assert_eq!(
      r#"<html><body><custom-element data-props-id="1"><div>1</div></custom-element></body></html>"#,
      rendering::server::render(doc)
    );
  }

  #[test]
  fn test_hydration_id() {
    fn test_component(cx: Scope) -> Node {
      fn inner_fn(cx: Scope) -> Node {
        Node::Element(Box::new(Element {
          name: "div".to_owned(),
          props: Default::default(),
          hydration_id: Scope::with(&cx, "1", ()).hydration_id(),
          children: Node::Text("HydrationID".to_string()),
        }))
      }

      Node::CustomElement(Box::new(CustomElement {
        name: "custom-element".to_owned(),
        props: cx.props.serialize_props(),
        hydration_id: cx.hydration_id(),
        children: inner_fn(cx),
      }))
    }

    let doc = Document {
      body: test_component(Scope::new("1", Rc::new(()), ())),
    };

    assert_eq!(
      r#"<html><body><custom-element data-hk="1"><div data-hk="1.1">HydrationID</div></custom-element></body></html>"#,
      rendering::server::render(doc)
    );
  }
}
