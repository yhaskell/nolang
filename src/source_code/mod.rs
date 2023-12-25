pub mod location;

pub use location::*;
use std::{fs::File, io::Read};

#[derive(Debug)]
pub struct SourceCode {
  pub code: String,
  line_breaks: Vec<usize>,
}

impl SourceCode {
  pub fn new(code: String) -> SourceCode {
    let mut lines = vec![];
    let mut position: usize = 0;

    for ch in code.chars() {
      if ch == '\n' {
        lines.push(position + 1);
      }

      position += 1;
    }

    SourceCode {
      code,
      line_breaks: lines,
    }
  }

  pub fn from_file(filename: String) -> Result<SourceCode, std::io::Error> {
    let mut file = File::open(filename)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(SourceCode::new(contents))
  }

  pub fn to_location(&self, position: usize) -> Option<Location> {
    if position > self.code.len() {
      return None;
    }

    let line = match self.line_breaks.binary_search(&position) {
      Ok(n) => n + 1,
      Err(n) => n,
    };

    let lb = if line == 0 { 0 } else { self.line_breaks[line - 1] };

    Some(Location::new(position, line, position - lb))
  }

  pub fn get_line(&self, line: usize) -> Option<String> {
    let line_start = if line == 0 { 0 } else { *self.line_breaks.get(line - 1)? };
    let line_end = self.line_breaks.get(line).map_or(self.code.len(), |e| *e);

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

    // since there are 4 lines, there should be 3 line breaks
    assert_eq!(sc.line_breaks.len(), 3);
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
