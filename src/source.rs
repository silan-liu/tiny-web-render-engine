// 输入源处理
pub struct SourceHelper {
  pub pos: usize,
  pub input: String,
}

impl SourceHelper {
  // 下一个字符
  pub fn next_char(&self) -> char {
    self.input[self.pos..].chars().next().unwrap()
  }

  // 是否以 str 开头
  pub fn starts_with(&self, s: &str) -> bool {
    self.input[self.pos..].starts_with(s)
  }

  // 是否结束
  pub fn eof(&self) -> bool {
    self.pos >= self.input.len()
  }

  // 消费单个字符
  pub fn consume_char(&mut self) -> char {
    let mut iter = self.input[self.pos..].char_indices();
    let (_, cur_char) = iter.next().unwrap();
    let (next_pos, _) = iter.next().unwrap_or((1, ' '));

    self.pos += next_pos;
    cur_char
  }

  // 循环消费字符，如果满足 test 条件
  pub fn consume_while<F>(&mut self, test: F) -> String
  where
    F: Fn(char) -> bool,
  {
    let mut result = String::new();
    while !self.eof() && test(self.next_char()) {
      result.push(self.consume_char())
    }

    result
  }

  // 跳过空白字符
  pub fn consume_whitespace(&mut self) {
    self.consume_while(char::is_whitespace);
  }
}
