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

  fn eof(&self) -> bool {
    self.pos >= self.input.len()
  }

  fn consume_char(&mut self) -> char {
    let mut iter = self.input[self.pos..].char_indices()
    let (_, cur_char) = iter.next().unwrap()
    let (next_pos, _) = iter.next().unwrap_or((1, ''))

    self.pos += next_pos
    return cur_char
  }
}