use std::collections::HashMap;

pub struct Trie {
  end: Option<usize>,
  children: HashMap<char, Trie>,
}

impl Trie {
  pub fn new() -> Trie {
    Trie {
      end: None,
      children: HashMap::new(),
    }
  }

  pub fn from_list(list: &[&str]) -> Trie {
    let mut trie = Trie::new();
    for word in list {
      trie.push(word);
    }
    trie
  }

  pub fn from_op_list() -> Trie {
    Trie::from_list(&[
      "=", "==", "!=", ">", "<", ">=", "<=", // comparison
      "+", "-", "", "/", "%", // base arithmetic
      "++", "--", // increment & decrement
      "+=", "-=", "=", "/=", "%=", // shorthand arithmetic
      "<<", ">>", "&", "|", "^", "~", // bit manipulation
      "&&", "||", "!", // logical
      "|>", "?", ":", "::", // to be determined if we need those
      "..", // range
      ".",  // member
    ])
  }

  fn push_internal(self: &mut Self, word: &str, len: usize, idx: usize) -> Option<usize> {
    if idx == len {
      self.end = Some(len);
      return self.end;
    };

    let letter = word.chars().nth(idx)?;

    if !self.children.contains_key(&letter) {
      self.children.insert(letter, Trie::new());
    }

    let trie = self.children.get_mut(&letter)?;

    trie.push_internal(word, len, idx + 1)
  }

  pub fn push(self: &mut Self, word: &str) -> bool {
    match self.push_internal(word, word.len(), 0) {
      Some(_) => true,
      _ => false,
    }
  }

  fn has_internal(&self, word: &str, len: usize, idx: usize) -> Option<usize> {
    if idx == len {
      self.end
    } else {
      let letter = word.chars().nth(idx)?;
      let subtrie = self.children.get(&letter)?;
      subtrie.has_internal(word, len, idx + 1)
    }
  }

  pub fn has(&self, word: &str) -> bool {
    self.has_internal(word, word.len(), 0).is_some()
  }

  fn has_prefix_internal(&self, word: &str, len: usize, idx: usize) -> Option<usize> {
    if idx == len {
      Some(len)
    } else {
      let letter = word.chars().nth(idx)?;
      let subtrie = self.children.get(&letter)?;
      subtrie.has_internal(word, len, idx + 1)
    }
  }

  pub fn has_prefix(&self, word: &str) -> bool {
    self.has_prefix_internal(word, word.len(), 0).is_some()
  }

  pub fn has_start(&self, ch: char) -> bool {
    self.children.contains_key(&ch)
  }

  pub fn get(&self, c: char) -> Option<&Trie> {
    self.children.get(&c)
  }

  pub fn is_leaf(&self) -> bool {
    self.end.is_some()
  }
}

#[cfg(test)]
mod test {
  use super::Trie;

  #[test]
  fn pushes_elements() {
    let mut trie = Trie::new();
    trie.push("abc");
    trie.push("abd");

    assert_eq!(trie.end, None);
    assert_eq!(
      trie.children.get(&'a').unwrap().children.get(&'b').unwrap().children.get(&'d').unwrap().end,
      Some(3)
    )
  }

  #[test]
  fn returns_has_for_elements() {
    let mut trie = Trie::new();
    trie.push("abc");
    trie.push("abd");

    assert_eq!(trie.has("abc"), true);
    assert_eq!(trie.has("abd"), true);
    assert_eq!(trie.has("adc"), false);
  }
  #[test]
  fn creates_from_list() {
    let trie = Trie::from_list(&["abc", "abd"]);

    assert_eq!(trie.has("abc"), true);
    assert_eq!(trie.has("abd"), true);
    assert_eq!(trie.has("adc"), false);
    assert_eq!(trie.has("abcd"), false);
    assert_eq!(trie.has("ab"), false);
  }
}
