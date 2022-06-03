use std::collections::HashMap;

// TODO: Find out how to make work the Node::Dyn*, those are dynamic components that should be able to be updated when their state changes
pub enum Node {
  DynChild(Box<dyn Fn() -> Node + 'static>),
  DynText(Box<dyn Fn() -> Node + 'static>),
  Element(Element),
  Fragment(Vec<Node>),
  None,
  Raw(String),
  Text(String),
}

impl std::fmt::Display for Node {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Node::DynChild(func) => write!(f, "{}", func()),
      Node::DynText(func) => write!(f, "{}", func()),
      Node::Element(e) => write!(f, "{}", e),
      Node::Fragment(children) => children.iter().try_for_each(|c| write!(f, "{}", c)),
      Node::None => Ok(()),
      Node::Raw(raw) => write!(f, "{}", raw),
      Node::Text(text) => write!(f, "{}", text),
    }
  }
}

// TODO: This will be a web_sys::Node
pub struct DomNode {}

pub trait IntoNode {
  fn into_node(self) -> Node;
}

pub struct Attributes {
  attrs_order: Vec<String>,
  attrs: HashMap<String, String>,
  classes: Vec<String>,
}

impl Attributes {
  pub fn new() -> Self {
    Self {
      attrs_order: vec![],
      attrs: HashMap::new(),
      classes: vec![],
    }
  }

  pub fn set_attr<K, V>(&mut self, key: K, value: V)
  where
    K: ToString,
    V: ToString,
  {
    let key = key.to_string();
    if !self.attrs_order.contains(&key) {
      self.attrs_order.push(key.clone())
    }

    self.attrs.insert(key, value.to_string());
  }

  fn set_class<V>(&mut self, value: V)
  where
    V: ToString,
  {
    self.classes.extend_from_slice(
      &value
        .to_string()
        .split(" ")
        .into_iter()
        .filter_map(|v| {
          let v = v.trim();
          if v.is_empty() {
            None
          } else {
            Some(v.to_string())
          }
        })
        .collect::<Vec<_>>(),
    );
    self.classes.dedup();
  }

  pub fn set_data<K, V>(&mut self, key: K, value: V)
  where
    K: ToString,
    V: ToString,
  {
    self.set_attr(format!("data-{}", key.to_string()), value);
  }
}

impl std::fmt::Display for Attributes {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "")
  }
}

pub struct Element {
  name: Option<String>,
  attributes: Attributes,
  children: Vec<Node>,
}

impl std::fmt::Display for Element {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.name.as_ref() {
      Some(name) => {
        write!(f, "<{}{}", name, &self.attributes)?;
        if self.children.is_empty() {
          write!(f, "/>")
        } else {
          write!(f, ">")?;
          self.children.iter().try_for_each(|c| write!(f, "{}", &c))?;
          write!(f, "</{}>", name)
        }
      }
      None => self.children.iter().try_for_each(|c| write!(f, "{}", &c)),
    }
  }
}

pub trait NodeBuilder {
  fn build(self) -> Node;
}

pub trait NodeGlobalAttributesBuilder {
  fn set_class<V>(self, value: V) -> Self
  where
    V: ToString;
  fn set_data<K, V>(self, key: K, value: V) -> Self
  where
    K: ToString,
    V: ToString;
  fn set_id<V>(self, value: V) -> Self
  where
    V: ToString;
}

pub trait NodeChildrenBuilder {
  fn add_child(self, child: Node) -> Self;
}

// TODO: Specialize the builder for each tag type, so we can validate the tag params
pub struct ElementBuilder {
  name: Option<String>,
  attributes: Attributes,
  children: Vec<Node>,
}

impl NodeGlobalAttributesBuilder for ElementBuilder {
  fn set_class<V>(mut self, value: V) -> Self
  where
    V: ToString,
  {
    self.attributes.set_class(value);
    self
  }

  fn set_data<K, V>(mut self, key: K, value: V) -> Self
  where
    K: ToString,
    V: ToString,
  {
    self.attributes.set_data(key, value);
    self
  }

  fn set_id<V>(mut self, value: V) -> Self
  where
    V: ToString,
  {
    self.attributes.set_attr("id", value);
    self
  }
}

impl NodeBuilder for ElementBuilder {
  fn build(self) -> Node {
    Node::Element(Element {
      name: self.name,
      attributes: self.attributes,
      children: self.children,
    })
  }
}

impl NodeChildrenBuilder for ElementBuilder {
  fn add_child(mut self, child: Node) -> Self {
    self.children.push(child);
    self
  }
}

pub fn element<N>(name: N) -> ElementBuilder
where
  N: ToString,
{
  ElementBuilder {
    name: Some(name.to_string()),
    attributes: Attributes::new(),
    children: vec![],
  }
}

pub struct FragmentBuilder {
  children: Vec<Node>,
}

impl NodeBuilder for FragmentBuilder {
  fn build(self) -> Node {
    Node::Fragment(self.children)
  }
}

impl NodeChildrenBuilder for FragmentBuilder {
  fn add_child(mut self, child: Node) -> Self {
    self.children.push(child);
    self
  }
}

pub fn fragment() -> FragmentBuilder {
  FragmentBuilder { children: vec![] }
}
