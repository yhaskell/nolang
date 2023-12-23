#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Location {
  pub position: usize,
  pub line: usize,
  pub offset: usize,
}

impl std::fmt::Display for Location {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}:{}", self.line, self.offset)
  }
}

impl Location {
  pub fn new(position: usize, line: usize, offset: usize) -> Location {
    Location {
      line,
      offset,
      position,
    }
  }
}
