use crate::ast::{Ast, Literal, Value};

pub fn compute(ast: &Ast) -> Result<f64, String> {
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
    Value::Identifier(_) => Err(format!("Identifiers are currently not supported")),
    Value::Unary(op, expr) => {
      let value = compute(expr)?;

      match op.as_str() {
        "+" => Ok(value),
        "-" => Ok(-value),
        other => Err(format!("{}: unknown operator", other)),
      }
    }
    Value::Binary(left, op, right) => {
      let left = compute(left)?;
      let right = compute(right)?;

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
    Value::Expression(e) => compute(e),
    Value::Assignment(_, _) => Err(format!("Assignments are currently not supported")),
  }
}
