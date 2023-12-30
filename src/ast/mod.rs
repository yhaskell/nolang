mod ast;
mod literal;
mod parser;

use crate::{source_code, tokeniser};
pub use ast::{Ast, ErrorCode, Value};
pub use literal::Literal;
pub use parser::from_tokens;

pub fn from_string(code: &str) -> Ast {
  let source_code = source_code::SourceCode::new(code.to_string());
  let tokens = tokeniser::from_source_code(&source_code);
  let ast = parser::from_tokens(tokens);

  ast
}
