use std::fmt::Display;

#[cfg(test)]
pub mod generators;

use crate::source_code::Location;

#[derive(Debug, PartialEq, Clone)]
pub enum ErrorCode {
  UnterminatedCharLiteral,
  EmptyCharLiteral,
  CharLiteralTooLong,
  BrokenUnicodeSequence,
  UnknownEscapeSequence,
  UnexpectedToken,
  BrokenStringLiteral,
  UnterminatedStringLiteral,
  IntLiteralTooLong,
  FloatLiteralTooLong,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenValue {
  CharLiteral(char),
  StringLiteral(String),
  IntLiteral(u128),
  FloatLiteral(f64),
  Identifier(String),
  Operator(String),
  Bracket(char),
  Error(String, ErrorCode),
}

#[derive(Debug, Clone)]
pub struct Token {
  pub value: TokenValue,
  pub start: Location,
  pub end: Location,
}

impl Display for Token {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.value)
  }
}

impl Display for TokenValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let value = match self {
      TokenValue::StringLiteral(s) => s.to_string(),
      TokenValue::CharLiteral(c) => c.to_string(),
      TokenValue::IntLiteral(i) => i.to_string(),
      TokenValue::FloatLiteral(f) => f.to_string(),
      TokenValue::Identifier(id) => id.to_string(),
      TokenValue::Operator(op) => op.to_string(),
      TokenValue::Bracket(b) => b.to_string(),
      TokenValue::Error(s, _err) => s.to_string(),
    };
    write!(f, "{}", value)
  }
}

impl Token {
  pub fn new(value: TokenValue, start: Location, end: Location) -> Token {
    Token { value, start, end }
  }

  pub fn is_identifier(&self) -> bool {
    match self.value {
      TokenValue::Identifier(_) => true,
      _ => false,
    }
  }

  pub fn is_literal(&self) -> bool {
    match self.value {
      TokenValue::CharLiteral(_)
      | TokenValue::StringLiteral(_)
      | TokenValue::IntLiteral(_)
      | TokenValue::FloatLiteral(_) => true,
      _ => false,
    }
  }
}
