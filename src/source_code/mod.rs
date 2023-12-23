pub mod location;

pub use location::*;
use std::{fs::File, io::Read};

#[derive(Debug)]
pub struct SourceCode {
  pub code: String,
  lines: Vec<usize>,
}

impl SourceCode {
  pub fn new(code: String) -> SourceCode {
    let mut lines = vec![0];
    let mut position: usize = 0;

    for ch in code.chars() {
      if ch == '\n' {
        lines.push(position + 1);
      }

      position += 1;
    }

    SourceCode { code, lines }
  }

  pub fn from_file(filename: String) -> Result<SourceCode, std::io::Error> {
    let mut file = File::open(filename)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(SourceCode::new(contents))
  }

  pub fn to_position(&self, location: Location) -> Option<usize> {
    let line_start = self.lines.get(location.line)?;

    Some(line_start + location.pos)
  }

  pub fn to_location(&self, position: usize) -> Option<Location> {
    if position > self.code.len() {
      return None;
    }

    let line = match self.lines.binary_search(&position) {
      Ok(n) => n,
      Err(n) => n,
    };

    Some(Location::new(line, position - line))
  }

  pub fn get_line(&self, line: usize) -> Option<String> {
    let line_start = *self.lines.get(line)?;
    let line_end = self.lines.get(line + 1).map_or(self.code.len(), |e| *e);

    Some(self.code[line_start..line_end].trim().to_string())
  }

  pub fn get_code_range(&self, from: usize, to: usize) -> Option<String> {
    let len = self.code.len();
    if from > len || to > len || to < from {
      None
    } else {
      Some(self.code[from..to].to_string())
    }
  }
}

#[cfg(test)]
mod test {
  use super::SourceCode;

  #[test]
  fn creates_source_code() {
    let code = "test test".to_string();
    let sc = SourceCode::new(code.clone());

    assert_eq!(code, sc.code);
  }

  #[test]
  fn calculates_lines_from_source_code() {
    let code = "line 1\nline 2\n\nline 3".to_string();
    let sc = SourceCode::new(code);

    assert_eq!(sc.lines.len(), 4);
  }

  #[test]
  fn returns_lines_from_source_code() {
    let code = "line 1\nline 2\n\nline 3".to_string();
    let sc = SourceCode::new(code);

    assert_eq!(sc.get_line(0), Some("line 1".to_string()));
    assert_eq!(sc.get_line(1), Some("line 2".to_string()));
    assert_eq!(sc.get_line(3), Some("line 3".to_string()));
  }

  #[test]
  fn returns_code_ranges() {
    let code = "line 1\nline 2\n\nline 3".to_string();
    let sc = SourceCode::new(code);

    assert_eq!(sc.get_code_range(3, 8), Some("e 1\nl".to_string()));
  }
}
