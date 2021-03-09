use std::collections:HashMap;

struct Parser {
  pos: usize,
  input: String,
}

impl Parser {
  fn next_char(&self) -> char {
    self.input[self.pos..].chars().next().unwrap()
  }

  fn starts_with(&self, s: &str) -> bool {
    self.input[self.pos..].starts_with(str)
  }

  // 是否结束
  fn eof(&self) -> bool {
    self.pos >= self.input.len()
  }

  // 消费单个字符
  fn consume_char(&mut self) -> char {
    let mut iter = self.input[self.pos..].char_indices()
    let (_, cur_char) = iter.next().unwrap()
    let (next_pos, _) = iter.next().unwrap_or((1, ''))

    self.pos += next_pos
    return cur_char
  }

  // 循环消费字符，如果满足 test 条件
  fn consume_while<F>(&mut self, test: F) -> String where F: Fn(char) -> bool {
    let mut result = String::new()
    while !self.eof() && test(self.next_char()) {
      result.push(self.consume_char())
    }

    return result
  }

  // 跳过空白字符
  fn consume_whitespace(&mut self) {
    self.consume_while(CharExt::is_whitespace);
  }

  // 解析标签名字
  fn parse_tag_name(&mut self) -> String {
    // 标签名字，a-z,A-Z,0-9 的组合
    self.consume_while(|c| match c {
      'a'...'z' | 'A'...'Z' | '0'...'9' => true,
      _ => false
    })
  }

  // 解析节点
  fn parse_node(&mut self) -> dom::Node {
    // 如果以 < 开头，则为标签，否则为文字
    match self.next_char() {
      '<' => self.parse_element(),
      _ => self.parse_text()
    }
  }

  // 解析文本
  fn parse_text(&mut self) -> dom::Node {
    // 获取文本内容，文本在标签中间，<p>hhh</p>
    let data = self.consume_while(|c| c != '<')
    dom::text(data)
  }

  // 解析标签
  // <div id="p1" class="c1"><p>hello</p></div>
  fn parse_element(&mut self) -> dom::Node {
    // 确保标签以 < 开头
    assert!(self.consume_char() == '<')

    // 解析标签名
    let tag_name = self.parse_tag_name()

    // 解析属性
    let attributes = self.parse_attributes()

    // 确保标签结束是 >
    assert!(self.consume_char() == '>')

    // 解析嵌套的子标签
    let children = self.parse_nodes()

    // </xx>
    assert!(self.consume_char() == '<')
    assert!(self.consume_char() == '/')

    // closing tag 的名字需和 opening tag 名字相同
    assert!(self.parse_tag_name() == tag_name)
    assert!(self.consume_char() == '>')

    return dom::elem(tag_name, attributes, children)
  }

  // 解析属性
  fn parse_attributes(&mut self) {
    let mut attributes = HashMap::new()
    
    loop {
      // 跳过空白字符
      self.consume_whitespace()

      // 如果到  opening tag 的末尾，结束
      if self.next_char() == '>' {
        break
      }

      // 解析属性
      let (name, value) = self.parse_attribute()

      // 插入字典
      attributes.insert(name, value)
    }

    return attributes
  }

  // 解析属性，key="xxx"
  fn parse_attribute(&mut self) {
    // 属性名
    let name = self.parse_tag_name()

    // 中间等号
    assert!(self.consume_char() == '=')

    // 属性值
    let value = self.parse_attr_value()
    return (name, value)
  }

  // 解析属性值，遇到 " 或 ' 结束
  fn parse_attr_value(&mut self) {
    let open_quote = self.consume_char()
    assert!(open_quote == '"' || open_quote == '\'')

    let value = self.consume_while(|c| c != open_quote)
    assert!(self.consume_char() == open_quote)
    return value
  }

  // 循环解析节点
  // <html></html>
  fn parse_nodes(&mut self) -> Vec<dom::Node> {
    let mut nodes = Vec::new()
    loop {
      self.consume_whitespace()

      if self.eof() || self.starts_with('</') {
        break
      }

      let node = self.parse_node()
      nodes.push(node)
    }

    return nodes
  }

  pub fn parse(source: String) -> dom::Node {
    let mut nodes = Parser { pos: 0, input: source}.parse_nodes()

    if nodes.len() == 1 {
      nodes.swap_remove(0)
    } else {
      dom::elem("html".to_string(), HashMap::new(), nodes)
    }
  }
}