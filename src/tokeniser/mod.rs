#[cfg(test)]
mod test;

mod check;
mod parsers;
mod token;
mod tokeniser;
mod trie;

use crate::source_code::SourceCode;

pub use token::{ErrorCode, Token, TokenValue};

use self::tokeniser::Tokeniser;

pub fn tokenise(code: &str) -> Vec<Token> {
  let source_code = SourceCode::new(code.to_string());
  let mut tokeniser = Tokeniser::new(source_code);

  tokeniser.parse()
}
