use std::collections::HashMap;

use crate::ast::{Ast, Literal, Value};

pub struct Computer {
  context: HashMap<String, f64>,
}

impl Computer {
  pub fn new() -> Self {
    Computer {
      context: HashMap::new(),
    }
  }

  pub fn iter(&self) -> std::collections::hash_map::Iter<'_, String, f64> {
    self.context.iter()
  }

  pub fn compute(&mut self, ast: &Ast) -> Result<f64, String> {
    match &ast.value {
      Value::Empty => Err(format!("Cannot compute empty value")),
      Value::Literal(l) => match l {
        Literal::Char(_) => Err(format!("Char literals are currently not supported")),
        Literal::Int(i) => {
          if let Ok(u) = i32::try_from(*i).and_then(|i| Ok(f64::from(i))) {
            Ok(u)
          } else {
            Err(format!("Error: {} is too big", i))
          }
        }
        Literal::Float(f) => Ok(*f),
        Literal::String(_) => Err(format!("String literals are currently not supported")),
      },
      Value::Identifier(id) => match self.context.get(id) {
        Some(value) => Ok(*value),
        None => Err(format!("{}: variable not found", id)),
      },
      Value::Unary(op, expr) => {
        let value = self.compute(expr)?;

        match op.as_str() {
          "+" => Ok(value),
          "-" => Ok(-value),
          other => Err(format!("{}: unknown operator", other)),
        }
      }
      Value::Binary(left, op, right) => {
        let left = self.compute(left)?;
        let right = self.compute(right)?;

        match op.as_str() {
          "+" => Ok(left + right),
          "-" => Ok(left - right),
          "*" => Ok(left * right),
          "/" => Ok(left / right),
          "%" => Ok(left % right),
          other => Err(format!("{}: unknown operator", other)),
        }
      }
      Value::Error(e) => Err(format!("{}", e)),
      Value::Expression(e) => self.compute(e),
      Value::Assignment(id, expr) => {
        let value = self.compute(&expr);
        let id = match &id.value {
          Value::Identifier(id) => id.clone(),
          _ => panic!("Should be assignment"),
        };

        match value {
          Ok(value) => {
            self.context.insert(id, value);
            Ok(value)
          }
          error => error,
        }
      }
    }
  }
}
