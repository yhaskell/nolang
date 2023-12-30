use crate::tokeniser::{Token, TokenValue};

use super::ast::{Ast, ErrorCode, Value};

pub struct Parser {
  tokens: Vec<Token>,
  position: usize,
  pstack: Vec<usize>,
}

macro_rules! can_consume {
  ($self: expr, $match: path) => {{
    let token = $self.tokens.get($self.position);
    if let Some(unwrapped) = token {
      match unwrapped.value.clone() {
        $match(_) => {
          $self.position += 1;
          token
        }
        _ => None,
      }
    } else {
      None
    }
  }};

  ($self: expr, $match: path[$value: expr]) => {{
    let token = $self.tokens.get($self.position);
    if let Some(unwrapped) = token {
      match unwrapped.value.clone() {
        $match(id) if id == $value => {
          $self.position += 1;
          token
        }
        _ => None,
      }
    } else {
      None
    }
  }};

  ($self: expr, $match: path { $cond: expr }) => {{
    let token = $self.tokens.get($self.position);
    if let Some(unwrapped) = token {
      match unwrapped.value.clone() {
        $match(val) if $cond(&val) => {
          $self.position += 1;
          token
        }
        _ => None,
      }
    } else {
      None
    }
  }};
}

impl Parser {
  pub fn new(tokens: Vec<Token>) -> Self {
    Parser {
      tokens,
      position: 0,
      pstack: vec![],
    }
  }

  fn current_token(&self) -> Option<&Token> {
    self.tokens.get(self.position)
  }

  fn start_node(&mut self) {
    self.pstack.push(self.position);
  }

  fn restore(&mut self) {
    self.position = self.pstack.pop().unwrap();
  }

  fn emit_node(&mut self, value: Value, advance: bool) -> Ast {
    let start = self.pstack.pop().unwrap();
    let end = if advance { self.position } else { self.position - 1 };

    if advance {
      self.position += 1;
    }

    Ast::from_value(value, self.tokens[start].start, self.tokens[end].end)
  }

  fn parse_atom(&mut self) -> Ast {
    self.start_node();

    let value = match self.current_token() {
      Some(t) if t.is_identifier() => Value::from_identifier(t),
      Some(t) if t.is_literal() => Value::from_literal(t),
      Some(_) => Value::from_error(ErrorCode::UnexpectedToken),
      _ => Value::from_error(ErrorCode::TokenExpected),
    };

    self.emit_node(value, true)
  }

  fn parse_term(&mut self) -> Ast {
    self.start_node();

    if let Some(_) = can_consume!(self, TokenValue::Bracket['(']) {
      let expr = self.parse_expression();
      let value = if let Some(_) = can_consume!(self, TokenValue::Bracket[')']) {
        Value::from_expression(expr)
      } else {
        Value::from_error(ErrorCode::RparenExpected)
      };
      self.emit_node(value, false)
    } else {
      self.parse_atom()
    }
  }

  fn parse_unary(&mut self) -> Ast {
    self.start_node();

    if let Some(op) = can_consume!(self, TokenValue::Operator{ |val| val == "+" || val == "-" }) {
      let op = op.value.clone();
      let expr = self.parse_term();
      let value = Value::from_unary(op, expr);

      self.emit_node(value, false)
    } else {
      self.parse_term()
    }
  }

  fn parse_multiplication_rest(&mut self, left: Ast) -> Ast {
    if let Some(op) = can_consume!(self, TokenValue::Operator { |val| "*/%".find(val).is_some() }) {
      let op = op.value.clone();
      let right = self.parse_unary();
      let end = right.end();
      let start = left.start();

      let value = Value::from_binary(left, op, right);

      let left = Ast::from_value(value, start, end);

      self.parse_multiplication_rest(left)
    } else {
      left
    }
  }

  fn parse_addition_rest(&mut self, left: Ast) -> Ast {
    if let Some(op) = can_consume!(self, TokenValue::Operator { |val| "+-".find(val).is_some() }) {
      let op = op.value.clone();
      let right = self.parse_multiplication();
      let end = right.end();
      let start = left.start();

      let value = Value::from_binary(left, op, right);

      let left = Ast::from_value(value, start, end);

      self.parse_addition_rest(left)
    } else {
      left
    }
  }

  fn parse_multiplication(&mut self) -> Ast {
    let left = self.parse_unary();

    self.parse_multiplication_rest(left)
  }

  fn parse_addition(&mut self) -> Ast {
    let left = self.parse_multiplication();

    self.parse_addition_rest(left)
  }

  fn parse_expression(&mut self) -> Ast {
    self.parse_addition()
  }

  fn parse_assignment_or_expression(&mut self) -> Ast {
    self.start_node();

    let id = can_consume!(self, TokenValue::Identifier);
    let eq = can_consume!(self, TokenValue::Operator["="]);

    if let [Some(id), Some(_)] = [id, eq] {
      let id = Ast::from_identifier(id);
      let expr = self.parse_expression();

      self.emit_node(Value::from_assignment(id, expr), false)
    } else {
      self.restore();

      self.parse_expression()
    }
  }

  /*
    Program ::= Line+
    Line ::= Assignment | Expression ";"
    Assignment ::= Identifier "=" Expression
    Expression ::= Multiplication AdditionTail
    AdditionTail ::= ["+" | "-"] Multiplication AdditionTail
    Multiplication ::= UnaryExpression MultiplicationTail
    MultiplicationTail ::= ["*" | "/" | "%"] UnaryExpression MultiplicationTail
    UnaryExpression ::= ["+" | "-"] Term
    Term ::= ("(" Expression ")") | Atom
    Atom ::= Identifier | Literal
  */
  pub fn parse(&mut self) -> Ast {
    self.parse_assignment_or_expression()
  }
}

pub fn from_tokens(tokens: Vec<Token>) -> Ast {
  let mut parser = Parser::new(tokens);

  parser.parse()
}

#[cfg(test)]
mod test {
  use crate::ast;

  macro_rules! test {
    ($name: ident, $expr: expr) => {
      #[test]
      pub fn $name() {
        let ast = ast::from_string($expr);

        assert_eq!(format!("{}", ast), $expr);
      }
    };

    ($name: ident, $left: expr => $right: expr) => {
      #[test]
      pub fn $name() {
        let ast = ast::from_string($left);

        assert_eq!(format!("{}", ast), $right);
      }
    };
  }

  test!(simple_assignment, "a = 42");
  test!(identifier, "a");
  test!(literal, "42");
  test!(term_in_brackets, "(a)");
  test!(assignment_with_brackets, "a=(a)" => "a = (a)");
  test!(unary, "+5");
  test!(assignment_with_unary, "a = -a");
  test!(nested_unary, "-(+a)");
  test!(multiplication, "a*b" => "a * b");
  test!(assigment_with_multiplication, "c = a * b");
  test!(multiplication_with_brackets, "a * (b * c)");
  test!(multiplication_multiple, "a * b * c");

  test!(addition, "a + b");
  test!(assigment_with_addition, "c = a + b");
  test!(addition_with_multiplication, "a + b * c");
  test!(multiplication_with_addition, "a * (b + c)");
  test!(multiplication_with_integer_addition, "2 * (2 + 3)");
  test!(addition_multiple, "a + b * c + d");
}
