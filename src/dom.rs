use std::collections::{HashMap, HashSet};
pub type AttrMap = HashMap<String, String>;

#[derive(Debug)]
pub struct Node {
  pub node_type: NodeType,
  pub children: Vec<Node>,
}

#[derive(Debug)]
pub enum NodeType {
  Element(ElementData),
  Text(String),
}

#[derive(Debug)]
pub struct ElementData {
  pub tag_name: String,
  pub attributes: AttrMap,
}

pub fn text(data: String) -> Node {
  Node {
    node_type: NodeType::Text(data),
    children: vec![],
  }
}

pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
  Node {
    node_type: NodeType::Element(ElementData {
      tag_name: name,
      attributes: attrs,
    }),
    children: children,
  }
}

impl ElementData {
  pub fn id(&self) -> Option<&String> {
    return self.attributes.get("id");
  }

  pub fn classes(&self) -> HashSet<&str> {
    match self.attributes.get("classes") {
      Some(classlist) => classlist.split(' ').collect(),
      None => HashSet::new(),
    }
  }
}
