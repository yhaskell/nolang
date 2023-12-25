use super::trie::Trie;
use once_cell::sync::Lazy;

pub static OP_LIST: Lazy<Trie> = Lazy::new(|| Trie::from_op_list());

pub fn is_bracket(c: char) -> bool {
  match c {
    '(' | ')' | '[' | ']' | '{' | '}' => true,
    _ => false,
  }
}

pub fn is_punctuation(c: char) -> bool {
  OP_LIST.has_start(c)
}

pub fn is_punctuation_or_whitespace(c: char) -> bool {
  return c.is_whitespace() || is_punctuation(c);
}
