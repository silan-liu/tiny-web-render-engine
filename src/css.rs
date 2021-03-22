use crate::source;

#[derive(Debug)]
pub struct StyleSheet {
  pub rules: Vec<Rule>,
}

#[derive(Debug)]
pub struct Rule {
  pub selectors: Vec<Selector>,
  pub declarations: Vec<Declaration>,
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

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
  Keyword(String),
  Length(f32, Unit),
  ColorValue(Color),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Unit {
  Px,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}

impl Copy for Color {}

// css 解析器
struct CSSParser {
  source_helper: source::SourceHelper,
}

pub type Specificity = (usize, usize, usize);

impl Selector {
  // 优先级：id > class > tag
  pub fn specificity(&self) -> Specificity {
    let Selector::Simple(ref simple) = *self;
    let a = simple.id.iter().count();
    let b = simple.class.len();
    let c = simple.tag_name.iter().count();

    (a, b, c)
  }
}

impl Value {
  pub fn to_px(&self) -> f32 {
    match *self {
      Value::Length(f, Unit::Px) => f,
      _ => 0.0,
    }
  }
}

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
  // 解析 css 规则
  fn parse_rules(&mut self) -> Vec<Rule> {
    let mut rules = Vec::new();

    loop {
      self.source_helper.consume_whitespace();
      if self.source_helper.eof() {
        break;
      }
      rules.push(self.parse_rule());
    }
    rules
  }

  //  解析单个 css 规则
  fn parse_rule(&mut self) -> Rule {
    Rule {
      selectors: self.parse_selectors(),
      declarations: self.parse_declarations(),
    }
  }

  // 解析组合选择器，以","分隔，返回数组
  fn parse_selectors(&mut self) -> Vec<Selector> {
    let mut selectors = Vec::new();
    loop {
      let simple_selector = self.parse_simple_selector();

      let selector = Selector::Simple(simple_selector);
      println!("selector:{:?}", selector);

      selectors.push(selector);

      self.source_helper.consume_whitespace();

      match self.source_helper.next_char() {
        '{' => break,
        ',' => {
          self.source_helper.consume_char();
          self.source_helper.consume_whitespace();
        }
        c => panic!("Unexpected char {} in selector list!", c),
      }
    }

    // 按优先级排序
    selectors.sort_by(|a, b| b.specificity().cmp(&a.specificity()));
    selectors
  }

  // 解析单个选择器
  fn parse_simple_selector(&mut self) -> SimpleSelector {
    let mut selector = SimpleSelector {
      tag_name: None,
      id: None,
      class: Vec::new(),
    };

    while !self.source_helper.eof() {
      match self.source_helper.next_char() {
        // id
        '#' => {
          self.source_helper.consume_char();
          selector.id = Some(self.parse_identifier());
        }

        // class
        '.' => {
          self.source_helper.consume_char();
          selector.class.push(self.parse_identifier());
        }

        // 通配符
        '*' => {
          self.source_helper.consume_char();
        }

        // tag
        c if valid_identifier_char(c) => {
          selector.tag_name = Some(self.parse_identifier());
        }

        _ => break,
      }
    }

    selector
  }

  // 解析字母数字-_组成的符号
  fn parse_identifier(&mut self) -> String {
    self.source_helper.consume_while(valid_identifier_char)
  }

  // 解析单个规则中的设置的所有属性
  fn parse_declarations(&mut self) -> Vec<Declaration> {
    assert_eq!(self.source_helper.consume_char(), '{');
    let mut declarations = Vec::new();
    loop {
      self.source_helper.consume_whitespace();
      if self.source_helper.next_char() == '}' {
        self.source_helper.consume_char();
        break;
      }

      let declaration = self.parse_declaration();
      declarations.push(declaration);
    }
    declarations
  }

  // 解析属性，键值对，margin-top: 12px;background-color:red
  fn parse_declaration(&mut self) -> Declaration {
    let property_name = self.parse_identifier();
    self.source_helper.consume_whitespace();
    assert_eq!(self.source_helper.consume_char(), ':');
    self.source_helper.consume_whitespace();

    let value = self.parse_value();
    self.source_helper.consume_whitespace();
    assert_eq!(self.source_helper.consume_char(), ';');

    Declaration {
      name: property_name,
      value: value,
    }
  }

  // 解析属性值，数字、色值、字符串
  fn parse_value(&mut self) -> Value {
    match self.source_helper.next_char() {
      '0'..='9' => self.parse_length(),
      '#' => self.parse_color(),
      _ => Value::Keyword(self.parse_identifier()),
    }
  }

  // 解析数字和单位
  fn parse_length(&mut self) -> Value {
    Value::Length(self.parse_float(), self.parse_unit())
  }

  // 解析浮点数
  fn parse_float(&mut self) -> f32 {
    let s = self.source_helper.consume_while(|c| match c {
      '0'..='9' | '.' => true,
      _ => false,
    });

    s.parse().unwrap()
  }

  // 解析单位
  fn parse_unit(&mut self) -> Unit {
    // &str
    let unit = self.parse_identifier().to_ascii_lowercase();
    match &*unit {
      "px" => Unit::Px,
      _ => panic!("Unexpected unit"),
    }
  }

  // 解析颜色值
  fn parse_color(&mut self) -> Value {
    assert_eq!(self.source_helper.consume_char(), '#');
    let color = Color {
      r: self.parse_hex_pair(),
      g: self.parse_hex_pair(),
      b: self.parse_hex_pair(),
      a: 255,
    };
    Value::ColorValue(color)
  }

  // 解析十六进制
  fn parse_hex_pair(&mut self) -> u8 {
    let s = &self.source_helper.input[self.source_helper.pos..self.source_helper.pos + 2];

    self.source_helper.pos += 2;
    u8::from_str_radix(s, 16).unwrap()
  }
}

// 有效的字符
fn valid_identifier_char(c: char) -> bool {
  match c {
    'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' => true,
    _ => false,
  }
}
