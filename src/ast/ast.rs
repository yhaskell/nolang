use super::literal::Literal;
use std::fmt::Display;

use crate::{
  source_code::Location,
  tokeniser::{Token, TokenValue},
};

#[derive(Debug)]
pub enum ErrorCode {
  TokenExpected,
  UnexpectedToken,
  RparenExpected,
  AtomExpected,
  LiteralExpected,
}

#[derive(Debug)]
pub enum Value {
  Empty,
  Literal(Literal),
  Identifier(String),
  Unary(String, Box<Ast>),
  Binary(Box<Ast>, String, Box<Ast>),
  Error(ErrorCode),
  Expression(Box<Ast>),
  Assignment(Box<Ast>, Box<Ast>),
}

impl Display for ErrorCode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let error = match self {
      ErrorCode::TokenExpected => "Expected Token",
      ErrorCode::RparenExpected => "Expected RPAREN",
      ErrorCode::AtomExpected => "Expected Identifier or Literal",
      ErrorCode::LiteralExpected => "Expected Literal",
      ErrorCode::UnexpectedToken => "Unexpected Token",
    };

    write!(f, "{}", error)
  }
}

impl Display for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let value = match self {
      Value::Empty => "(empty)".to_string(),
      Value::Literal(t) => t.to_string(),
      Value::Identifier(s) => s.to_string(),
      Value::Unary(op, e) => format!("{}{}", op, e),
      Value::Binary(left, op, right) => format!("{} {} {}", left, op, right),
      Value::Expression(e) => format!("({})", e.to_string()),
      Value::Error(err) => format!("{}", err),
      Value::Assignment(id, expr) => format!("{} = {}", id, expr),
    };
    write!(f, "{}", value)
  }
}

impl Value {
  pub fn from_literal(token: &Token) -> Self {
    let value = match token.value.clone() {
      TokenValue::CharLiteral(c) => Literal::Char(c),
      TokenValue::IntLiteral(i) => Literal::Int(i),
      TokenValue::FloatLiteral(f) => Literal::Float(f),
      TokenValue::StringLiteral(s) => Literal::String(s),
      _ => panic!("{} cannot be used to produce a literal", token),
    };

    Value::Literal(value)
  }

  pub fn from_identifier(token: &Token) -> Self {
    let value = match token.value.clone() {
      TokenValue::Identifier(id) => id,
      _ => panic!("{} cannot be used to produce a identifier", token),
    };

    Value::Identifier(value)
  }

  pub fn from_unary(op: TokenValue, expr: Ast) -> Self {
    let op = match op {
      TokenValue::Operator(op) => op,
      _ => panic!("{} cannot be used to produce an unary expression", op),
    };

    Value::Unary(op, Box::new(expr))
  }

  pub fn from_binary(left: Ast, op: TokenValue, right: Ast) -> Self {
    let op = match op {
      TokenValue::Operator(op) => op,
      _ => panic!("{} cannot be used to produce an binary expresseion", op),
    };

    Value::Binary(Box::new(left), op, Box::new(right))
  }

  pub fn from_error(error: ErrorCode) -> Self {
    Value::Error(error)
  }

  pub fn from_assignment(id: Ast, expr: Ast) -> Self {
    Value::Assignment(Box::new(id), Box::new(expr))
  }

  pub fn from_expression(expr: Ast) -> Self {
    Value::Expression(Box::new(expr))
  }
}

#[derive(Debug)]
pub struct Ast {
  pub value: Value,
  pub start: Location,
  pub end: Location,
}

impl Display for Ast {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.value.fmt(f)
  }
}

impl Ast {
  pub fn from_identifier(id: &Token) -> Ast {
    let value = Value::from_identifier(id);

    Ast {
      value,
      start: id.start,
      end: id.end,
    }
  }

  pub fn from_value(value: Value, start: Location, end: Location) -> Ast {
    Ast {
      value,
      start: start,
      end: end,
    }
  }

  pub fn start(&self) -> Location {
    self.start.clone()
  }
  pub fn end(&self) -> Location {
    self.end.clone()
  }
}
