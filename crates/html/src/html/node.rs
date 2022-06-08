use std::sync::atomic::{AtomicUsize, Ordering};

use http::{StatusCode, Uri};

use crate::{
  html::{Doctype, Element},
  Children,
};

// TODO: Migrate from `From<X> for Node` to `IntoNode`
pub trait IntoNode {
  fn into_node(self) -> Node;
}

static NODE_ID_POOL: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, PartialEq)]
pub struct NodeId {
  id: usize,
}

impl NodeId {
  pub fn new() -> Self {
    let id = NODE_ID_POOL.fetch_add(1, Ordering::SeqCst);
    Self { id }
  }
}

#[derive(Debug, Clone)]
pub enum Node {
  Comment(Children),
  CustomElement,
  Document(Doctype, Children),
  Element(Element, NodeId),
  Fragment(Children, NodeId),
  None,
  Raw(String),
  Redirect(StatusCode, Uri),
  Text(String),
}

impl Node {
  pub fn get_redirect(&self) -> Option<(StatusCode, Uri)> {
    match self {
      Node::Element(el, _) => match el.children.iter().find(|c| c.get_redirect().is_some()) {
        Some(node) => node.get_redirect(),
        None => None,
      },
      Node::Fragment(children, _) => match children.iter().find(|c| c.get_redirect().is_some()) {
        Some(node) => node.get_redirect(),
        None => None,
      },
      Node::Redirect(status, uri) => Some((status.clone(), uri.clone())),
      _ => None,
    }
  }

  /// Normalize will do multiple operations over the current node, one of the notable
  /// ones is to move the nodes with a `slot` attribute into the corresponding `slot`
  /// tag.
  pub fn normalize(mut self) -> Self {
    let mut remove_slots = vec![];

    self.find_slots().iter().for_each(|(el, id)| {
      if let Some(name) = el.attr("name") {
        if self.fill_tag_with_slot(&name.to_string(), &el) {
          remove_slots.push(id.clone());
        } else {
          println!(
            r#"⚠️ No tag with attribute slot="{}" found"#,
            name.to_string()
          );
        }
      }
    });

    remove_slots.iter().for_each(|id| self.remove_node(id));

    self
  }

  pub fn strip_slots(mut self) -> Self {
    self
      .find_slots()
      .iter()
      .for_each(|(_, id)| self.remove_node(id));

    self
  }

  fn fill_tag_with_slot(&mut self, name: &str, slot: &Element) -> bool {
    fn iter_children(children: &mut Children, name: &str, slot: &Element) -> bool {
      let mut res = false;
      for node in children.iter_mut() {
        if node.fill_tag_with_slot(name, slot) {
          res = true;
        }
      }
      res
    }
    let filled_slot = match self {
      Node::Document(_, children) => iter_children(children, name, slot),
      Node::Element(el, _) => {
        let res_a = if el.has_attr_value("slot", name) {
          el.children = slot.children.clone();
          true
        } else {
          false
        };

        let res_b = iter_children(&mut el.children, name, slot);
        res_a || res_b
      }
      Node::Fragment(children, _) => iter_children(children, name, slot),
      _ => false,
    };

    filled_slot
  }

  fn remove_node(&mut self, cid: &NodeId) {
    fn match_id(node: &Node, cid: &NodeId) -> bool {
      match node {
        Node::Element(_, id) => id == cid,
        Node::Fragment(_, id) => id == cid,
        _ => false,
      }
    }
    fn iter_children(children: &mut Children, cid: &NodeId) {
      let mut index = None;
      for (i, node) in children.iter_mut().enumerate() {
        if match_id(node, cid) {
          index = Some(i);
          break;
        } else {
          node.remove_node(cid);
        }
      }

      if let Some(index) = index {
        children.remove(index);
      }
    }
    match self {
      Node::Document(_, children) => iter_children(children, cid),
      Node::Element(el, _) => iter_children(&mut el.children, cid),
      Node::Fragment(children, _) => iter_children(children, cid),
      _ => {}
    }
  }

  fn find_slots(&self) -> Vec<(Element, NodeId)> {
    fn find_in_children(children: &Children) -> Vec<(Element, NodeId)> {
      children.iter().flat_map(|c| c.find_slots()).collect()
    }

    match self {
      Node::Document(_, children) => find_in_children(children),
      Node::Element(el, id) => {
        if el.name == "slot" {
          vec![(el.clone(), id.clone())]
        } else {
          find_in_children(&el.children)
        }
      }
      Node::Fragment(children, _) => find_in_children(children),
      _ => vec![],
    }
  }
}

// impl<S, E> From<Result<S, E>> for Node
// where
//   S: Into<Node>,
//   E: Into<Node>,
// {
//   fn from(item: Result<S, E>) -> Self {
//     match item {
//       Ok(success) => success.into(),
//       Err(err) => err.into(),
//     }
//   }
// }

impl ToString for Node {
  fn to_string(&self) -> String {
    match self {
      Node::Comment(_) => "Comment",
      Node::CustomElement => "CustomElement",
      Node::Document(_, _) => "Document",
      Node::Element(_, _) => "Element",
      Node::Fragment(_, _) => "Fragment",
      Node::None => "None",
      Node::Raw(_) => "Raw",
      Node::Redirect(_, _) => "Redirect",
      Node::Text(_) => "Text",
    }
    .to_string()
  }
}

impl From<Vec<Node>> for Node {
  fn from(children: Vec<Node>) -> Node {
    Node::Fragment(Children { children }, NodeId::new())
  }
}

impl From<Option<Node>> for Node {
  fn from(item: Option<Node>) -> Node {
    match item {
      Some(node) => node,
      None => Node::None,
    }
  }
}

impl From<Option<Vec<Node>>> for Node {
  fn from(item: Option<Vec<Node>>) -> Node {
    match item {
      Some(children) => Node::Fragment(Children { children }, NodeId::new()),
      None => Node::None,
    }
  }
}

macro_rules! impl_renderable {
  ($($t:ty),*) => {
    $(
      impl From<$t> for Node {
        fn from(item: $t) -> Node {
          Node::Text(item.to_string())
        }
      }

      impl From<& $t> for Node {
        fn from(item: & $t) -> Node {
          Node::Text(item.to_string())
        }
      }

      impl From<Option<$t>> for Node {
        fn from(item: Option<$t>) -> Node {
          match item.as_ref() {
            Some(item) => Node::Text(item.to_string()),
            None => Node::None,
          }
        }
      }

      impl From<Option<& $t>> for Node {
        fn from(item: Option<& $t>) -> Node {
          match item {
            Some(item) => Node::Text(item.to_string()),
            None => Node::None,
          }
        }
      }
    )*
  };
}

impl_renderable!(
  String, &str, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64
);
