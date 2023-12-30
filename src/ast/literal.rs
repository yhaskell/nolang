use std::fmt::Display;

#[derive(Debug)]
pub enum Literal {
  Char(char),
  Int(u128),
  Float(f64),
  String(String),
}

impl Display for Literal {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let r = match self {
      Literal::Char(c) => c.to_string(),
      Literal::Int(u) => u.to_string(),
      Literal::Float(f) => f.to_string(),
      Literal::String(s) => s.to_string(),
    };
    write!(f, "{}", r)
  }
}
