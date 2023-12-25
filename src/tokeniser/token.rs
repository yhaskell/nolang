use crate::source_code::Location;

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum TokenValue {
  StringLiteral(String),
  CharLiteral(char),
  IntLiteral(u128),
  FloatLiteral(f64),
  Identifier(String),
  Operator(String),
  Bracket(char),
  Error(String, ErrorCode),
}

#[derive(Debug)]
pub struct Token {
  pub value: TokenValue,
  pub start: Location,
  pub end: Location,
}

impl Token {
  pub fn new(value: TokenValue, start: Location, end: Location) -> Token {
    Token { value, start, end }
  }
}
