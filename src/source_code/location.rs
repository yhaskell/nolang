#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Location {
  pub line: usize,
  pub pos: usize,
}

impl std::fmt::Display for Location {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}:{}", self.line, self.pos)
  }
}

impl Location {
  pub fn new(line: usize, pos: usize) -> Location {
    Location { line, pos }
  }
}
