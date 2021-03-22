use crate::css::{Rule, Selector, SimpleSelector, Specificity, StyleSheet, Value};
use crate::dom::{ElementData, Node, NodeType};
use std::collections::HashMap;

type PropertyMap = HashMap<String, Value>;

#[derive(Debug)]
pub struct StyleNode<'a> {
  pub node: &'a Node,
  pub specified_values: PropertyMap,
  pub children: Vec<StyleNode<'a>>,
}

#[derive(Debug)]
pub enum Display {
  Inline,
  Block,
  None,
}

// 节点与选择器是否匹配
fn matches(elem: &ElementData, selector: &Selector) -> bool {
  match *selector {
    Selector::Simple(ref simple_selector) => match_simple_selector(elem, simple_selector),
  }
}

// 匹配逻辑，只要有一个不满足，则返回 false
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

impl<'a> StyleNode<'a> {
  pub fn value(&self, name: &str) -> Option<Value> {
    self.specified_values.get(name).cloned()
  }

  // 先查找 name 的值，不存在，则查找 fallback_name 的值，若仍然不存在，则返回 default
  pub fn lookup(&self, name: &str, fallback_name: &str, default: Value) -> Value {
    self
      .value(name)
      .unwrap_or_else(|| self.value(fallback_name).unwrap_or_else(|| default.clone()))
  }

  // 根据 display 的值返回对应类型
  pub fn display(&self) -> Display {
    match self.value("display") {
      Some(Value::Keyword(s)) => match &*s {
        "block" => Display::Block,
        "none" => Display::None,
        _ => Display::Inline,
      },
      _ => Display::Inline,
    }
  }
}

type MatchedRule<'a> = (Specificity, &'a Rule);

// 检查节点是否满足样式规则，返回规则
fn match_rule<'a>(elem: &ElementData, rule: &'a Rule) -> Option<MatchedRule<'a>> {
  rule
    .selectors
    .iter()
    .find(|selector| matches(elem, selector))
    .map(|selector| (selector.specificity(), rule))
}

// 计算出满足节点的所有样式
fn matching_rules<'a>(elem: &ElementData, stylesheet: &'a StyleSheet) -> Vec<MatchedRule<'a>> {
  stylesheet
    .rules
    .iter()
    .filter_map(|rule| match_rule(elem, rule))
    .collect()
}

// 将样式转换为 map
fn specified_values<'a>(elem: &ElementData, stylesheet: &'a StyleSheet) -> PropertyMap {
  let mut values = HashMap::new();
  let mut rules = matching_rules(elem, stylesheet);

  // [((1,0,0), rule1), ((0,1,1), rule2)] -> [((0,1,1), rule2), ((1,0,0), rule1)]
  // 从低优先级 -> 高优先级排序，这样在放入 map 时，高优先级会覆盖低优先级
  rules.sort_by(|&(a, _), &(b, _)| a.cmp(&b));

  for (_, rule) in rules {
    for declaration in &rule.declarations {
      values.insert(declaration.name.clone(), declaration.value.clone());
    }
  }

  return values;
}

// 生成渲染树
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
