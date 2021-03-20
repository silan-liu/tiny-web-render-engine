use crate::css::{Rule, Selector, SimpleSelector, Specificity, StyleSheet, Value};
use crate::dom::{ElementData, Node, NodeType};
use std::collections::HashMap;

type PropertyMap = HashMap<String, Value>;

pub struct StyleNode<'a> {
  pub node: &'a Node,
  pub specified_values: PropertyMap,
  pub children: Vec<StyleNode<'a>>,
}

// 节点与选择器是否匹配
fn matches(elem: &ElementData, selector: &Selector) -> bool {
  match *selector {
    Selector::Simple(ref simple_selector) => match_simple_selector(elem, simple_selector),
  }
}

// 匹配逻辑
fn match_simple_selector(elem: &ElementData, selector: &SimpleSelector) -> bool {
  // 检查 tag
  if selector.tag_name.iter().any(|name| elem.tag_name != *name) {
    return false;
  }

  // 检查 id
  if selector.id.iter().any(|id| elem.id() != Some(id)) {
    return false;
  }

  // 检查 classes
  if selector
    .class
    .iter()
    .any(|class| !elem.classes().contains(&**class))
  {
    return false;
  }

  return true;
}

impl<'a> StyleNode<'a> {}

type MatchedRule<'a> = (Specificity, &'a Rule);

fn match_rule<'a>(elem: &ElementData, rule: &'a Rule) -> Option<MatchedRule<'a>> {
  rule
    .selectors
    .iter()
    .find(|selector| matches(elem, selector))
    .map(|selector| (selector.specificity(), rule))
}

fn matching_rules<'a>(elem: &ElementData, stylesheet: &'a StyleSheet) -> Vec<MatchedRule<'a>> {
  stylesheet
    .rules
    .iter()
    .filter_map(|rule| match_rule(elem, rule))
    .collect()
}

fn specified_values<'a>(elem: &ElementData, stylesheet: &'a StyleSheet) -> PropertyMap {
  let mut values = HashMap::new();
  let mut rules = matching_rules(elem, stylesheet);

  rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));

  for (_, rule) in rules {
    for declaration in &rule.declarations {
      values.insert(declaration.name.clone(), declaration.value.clone());
    }
  }

  return values;
}

pub fn style_tree<'a>(root: &'a Node, stylesheet: &'a StyleSheet) -> StyleNode<'a> {
  StyleNode {
    node: root,
    specified_values: match root.node_type {
      NodeType::Element(ref elem) => specified_values(elem, stylesheet),
      NodeType::Text(_) => HashMap::new(),
    },
    children: root
      .children
      .iter()
      .map(|child| style_tree(child, stylesheet))
      .collect(),
  }
}
