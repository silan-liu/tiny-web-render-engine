use crate::dom;
use crate::source;
use std::collections::HashMap;

pub fn parse(source: String) -> dom::Node {
  let source_helper = source::SourceHelper {
    pos: 0,
    input: source,
  };

  let mut parser = HtmlParser {
    source_helper: source_helper,
  };

  let mut nodes = parser.parse_nodes();

  if nodes.len() == 1 {
    println!("has root node!");
    nodes.swap_remove(0)
  } else {
    println!("has no root node!");
    dom::elem("html".to_string(), HashMap::new(), nodes)
  }
}

struct HtmlParser {
  source_helper: source::SourceHelper,
}

impl HtmlParser {
  // fn next_char(&self) -> char {
  //   self.source_helper.next_char()
  // }

  // fn starts_with(&self, s: &str) -> bool {
  //   self.source_helper.starts_with(s)
  // }

  // // 是否结束
  // fn eof(&self) -> bool {
  //   self.source_helper.eof()
  // }

  // // 消费单个字符
  // fn consume_char(&mut self) -> char {
  //   self.source_helper.consume_char()
  // }

  // // 循环消费字符，如果满足 test 条件
  // fn consume_while<F>(&mut self, test: F) -> String
  // where
  //   F: Fn(char) -> bool,
  // {
  //   self.source_helper.consume_while(test)
  // }

  // // 跳过空白字符
  // fn consume_whitespace(&mut self) {
  //   self.source_helper.consume_whitespace()
  // }

  // 解析标签名字
  fn parse_tag_name(&mut self) -> String {
    // 标签名字，a-z,A-Z,0-9 的组合
    self.source_helper.consume_while(|c| match c {
      'a'..='z' | 'A'..='Z' | '0'..='9' => true,
      _ => false,
    })
  }

  // 解析节点
  fn parse_node(&mut self) -> dom::Node {
    // 如果以 < 开头，则为标签，否则为文字
    match self.source_helper.next_char() {
      '<' => self.parse_element(),
      _ => self.parse_text(),
    }
  }

  // 解析文本
  fn parse_text(&mut self) -> dom::Node {
    // 获取文本内容，文本在标签中间，<p>hhh</p>
    let data = self.source_helper.consume_while(|c| c != '<');
    dom::text(data)
  }

  // 解析标签
  // <div id="p1" class="c1"><p>hello</p></div>
  fn parse_element(&mut self) -> dom::Node {
    // 确保标签以 < 开头
    assert!(self.source_helper.consume_char() == '<');

    // 解析标签名
    let tag_name = self.parse_tag_name();
    println!("tag_name:{}", tag_name);

    // 解析属性
    let attributes = self.parse_attributes();

    // 确保标签结束是 >
    assert!(self.source_helper.consume_char() == '>');

    // 解析嵌套的子标签
    let children = self.parse_nodes();

    // </xx>
    assert!(self.source_helper.consume_char() == '<');
    assert!(self.source_helper.consume_char() == '/');

    // closing tag 的名字需和 opening tag 名字相同
    assert!(self.parse_tag_name() == tag_name);
    assert!(self.source_helper.consume_char() == '>');

    dom::elem(tag_name, attributes, children)
  }

  // 解析属性
  fn parse_attributes(&mut self) -> dom::AttrMap {
    let mut attributes = HashMap::new();
    loop {
      // 跳过空白字符
      self.source_helper.consume_whitespace();

      // 如果到  opening tag 的末尾，结束
      if self.source_helper.next_char() == '>' {
        break;
      }

      // 解析属性
      let (name, value) = self.parse_attribute();

      // 插入字典
      attributes.insert(name, value);
    }

    attributes
  }

  fn parse_attribute(&mut self) -> (String, String) {
    println!("parse_attribute");
    // 属性名
    let name = self.parse_tag_name();

    // 中间等号
    assert!(self.source_helper.consume_char() == '=');

    // 属性值
    let value = self.parse_attr_value();
    (name, value)
  }

  // 解析属性值，遇到 " 或 ' 结束
  fn parse_attr_value(&mut self) -> String {
    let open_quote = self.source_helper.consume_char();
    assert!(open_quote == '"' || open_quote == '\'');

    let value = self.source_helper.consume_while(|c| c != open_quote);
    assert!(self.source_helper.consume_char() == open_quote);
    value
  }

  // 循环解析节点
  // <html></html>
  fn parse_nodes(&mut self) -> Vec<dom::Node> {
    let mut nodes = Vec::new();
    loop {
      self.source_helper.consume_whitespace();

      // "</" 的判断，是为了找嵌套标签时，跳出。比如，<html></html>，在解析完<html>后，会重新调用 parse_nodes 解析子标签
      // 这时字符串是  </html>。
      if self.source_helper.eof() || self.source_helper.starts_with("</") {
        break;
      }

      let node = self.parse_node();
      nodes.push(node);
    }

    nodes
  }
}
