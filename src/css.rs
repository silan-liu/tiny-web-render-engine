use crate::source;

#[derive(Debug)]
pub struct StyleSheet {
  pub rules: Vec<Rule>,
}

#[derive(Debug)]
pub struct Rule {
  pub selectors: Vec<Selector>,
  pub declarations: Vec<Selector>,
}

#[derive(Debug)]
pub enum Selector {
  Simple(SimpleSelector),
}

#[derive(Debug)]
pub struct SimpleSelector {
  pub tag_name: Option<String>,
  pub id: Option<String>,
  pub class: Vec<String>,
}

#[derive(Debug)]
pub struct Declaration {
  pub name: String,
  pub value: Value,
}

#[derive(Debug)]
pub enum Value {
  Keyword(String),
  Length(f32, Unit),
  ColorValue(Color),
}

#[derive(Debug)]
pub enum Unit {
  Px,
}

#[derive(Debug)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}

// css 解析器
struct CSSParser {
  source_helper: source::SourceHelper,
}

pub type Specificity = (usize, usize, usize);

impl Selector {}

impl Value {}

// 对外提供的解析方法
pub fn parse(source: String) -> StyleSheet {
  let source_helper = source::SourceHelper {
    pos: 0,
    input: source,
  };

  let mut parser = CSSParser {
    source_helper: source_helper,
  };

  let rules = parser.parse_rules();
  StyleSheet { rules: rules }
}

// 解析器
impl CSSParser {
  fn parse_rules(&mut self) -> Vec<Rule> {
    let mut rules = Vec::new();

    self.source_helper.consume_whitespace();
    rules.push(self.parse_rule());
    rules
  }

  fn parse_rule(&mut self) -> Rule {
    Rule {
      selectors: Vec::new(),
      declarations: Vec::new(),
    }
  }
}
