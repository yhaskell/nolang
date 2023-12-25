#[cfg(test)]
mod test;

mod token;
mod trie;

use crate::source_code::SourceCode;
use once_cell::sync::Lazy;
use std::str::FromStr;
use token::{ErrorCode, Token, TokenValue};

use trie::Trie;

// static OP_LIST: Trie = Tokeniser::make_op_list();
static OP_LIST: Lazy<Trie> = Lazy::new(|| Trie::from_op_list());

pub struct Tokeniser {
  source_code: SourceCode,
  position: usize,
  current_token_start: usize,
}

macro_rules! select {
  ($ch: expr, $success: literal, $failure: literal) => {
    if $ch == '"' {
      $success
    } else {
      $failure
    }
  };
}

impl Tokeniser {
  pub fn new(source_code: SourceCode) -> Tokeniser {
    Tokeniser {
      source_code,
      position: 0,
      current_token_start: 0,
    }
  }

  pub fn parse_symbol_in_string(
    s: &str,
    pos: usize,
    parse_unicode: bool,
  ) -> Result<(char, usize), ErrorCode> {
    match s.chars().nth(pos) {
      None => Err(ErrorCode::BrokenStringLiteral),
      Some('\\') => match s.chars().nth(pos + 1) {
        None => Err(ErrorCode::UnterminatedCharLiteral),
        Some(c) => match c {
          'n' => Ok(('\n', pos + 2)),
          'r' => Ok(('\r', pos + 2)),
          't' => Ok(('\t', pos + 2)),
          '\'' => Ok(('\'', pos + 2)),
          '\\' => Ok(('\\', pos + 2)),
          'u' if !parse_unicode || s.len() < pos + 6 => Err(ErrorCode::BrokenUnicodeSequence),
          'u' => match u32::from_str_radix(&s[pos + 2..pos + 6], 16) {
            Err(_) => Err(ErrorCode::BrokenUnicodeSequence),
            Ok(u) => match char::from_u32(u) {
              Some(c) => Ok((c, pos + 6)),
              None => Err(ErrorCode::BrokenUnicodeSequence),
            },
          },
          _ => Err(ErrorCode::UnknownEscapeSequence),
        },
      },
      Some(c) => Ok((c, pos + 1)),
    }
  }

  pub fn parse_string(s: String) -> Result<String, ErrorCode> {
    let mut result = String::new();

    let len = s.len();
    let mut pos = 0;
    while pos < len {
      match Tokeniser::parse_symbol_in_string(&s, pos, true) {
        Err(e) => {
          return Err(e);
        }
        Ok((c, next)) => {
          result.push(c);
          pos = next;
        }
      };
    }
    Ok(result)
  }

  pub fn is_bracket(c: char) -> bool {
    match c {
      '(' | ')' | '[' | ']' | '{' | '}' => true,
      _ => false,
    }
  }

  pub fn is_punctuation(c: char) -> bool {
    OP_LIST.has_start(c)
  }

  pub fn is_punctuation_or_whitespace(c: char) -> bool {
    return c.is_whitespace() || Tokeniser::is_punctuation(c);
  }

  pub fn parse_int(self: &mut Self, radix: u32) -> Token {
    self.commit_token(|s| {
      match u128::from_str_radix(
        if radix == 16 {
          &s.trim_start_matches("0x")
        } else {
          &s
        },
        radix,
      ) {
        Ok(n) => TokenValue::IntLiteral(n),
        _ => TokenValue::Error(s, ErrorCode::IntLiteralTooLong),
      }
    })
  }

  pub fn parse_float(self: &mut Self) -> Token {
    self.commit_token(|s| match f64::from_str(&s) {
      Ok(n) => TokenValue::FloatLiteral(n),
      _ => TokenValue::Error(s, ErrorCode::FloatLiteralTooLong),
    })
  }

  pub fn start_token(self: &mut Self) {
    self.current_token_start = self.position;
  }

  pub fn commit_token<F>(self: &mut Self, value: F) -> Token
  where
    F: FnOnce(String) -> TokenValue,
  {
    let begin = self
      .source_code
      .code
      .char_indices()
      .nth(self.current_token_start)
      .map_or(self.source_code.code.len(), |e| e.0);
    let end = self
      .source_code
      .code
      .char_indices()
      .nth(self.position)
      .map_or(self.source_code.code.len(), |e| e.0);

    let value = value(self.source_code.code[begin..end].to_string());
    Token::new(
      value,
      self
        .source_code
        .to_location(self.current_token_start)
        .unwrap(),
      self.source_code.to_location(self.position).unwrap(),
    )
  }

  pub fn consume_identifier(self: &mut Self) -> Token {
    self.start_token();
    while let Some(c) = self.get_char(self.position) {
      if c.is_alphanumeric() || c == '_' {
        self.position += 1;
      } else {
        break;
      }
    }

    self.commit_token(|s| TokenValue::Identifier(s))
  }

  pub fn get_char(&self, position: usize) -> Option<char> {
    self.source_code.code.chars().nth(position)
  }

  pub fn consume_char_literal(self: &mut Self) -> Token {
    self.start_token();
    let mut closed = false;
    self.position += 1;

    let mut escape = false;
    while let Some(c) = self.get_char(self.position) {
      if c == '\n' {
        return self.commit_token(|s| TokenValue::Error(s, ErrorCode::UnterminatedCharLiteral));
      } else if c == '\'' && !escape {
        closed = true;
        break;
      } else if c == '\\' {
        escape = !escape;
      } else {
        escape = false;
      }
      self.position += 1;
    }

    self.position += 1;

    match closed {
      false => self.commit_token(|s| TokenValue::Error(s, ErrorCode::UnterminatedCharLiteral)),
      true => self.commit_token(|s| match s.chars().count() {
        0 | 1 => unreachable!("Cannot happen"),
        2 => TokenValue::Error(s, ErrorCode::EmptyCharLiteral),
        3 => TokenValue::CharLiteral(s.chars().nth(1).unwrap()),
        len => match Tokeniser::parse_symbol_in_string(&s, 1, true) {
          Err(e) => TokenValue::Error(s, e),
          Ok((c, next)) => {
            if next + 1 < len {
              TokenValue::Error(s, ErrorCode::CharLiteralTooLong)
            } else {
              TokenValue::CharLiteral(c)
            }
          }
        },
      }),
    }
  }

  pub fn consume_string_literal(self: &mut Self) -> Token {
    self.start_token();
    let mut state = 0;

    while let Some(c) = self.get_char(self.position) {
      if c == '\n' && state != 4 {
        return self.commit_token(|s| TokenValue::Error(s, ErrorCode::UnterminatedStringLiteral));
      };
      state = match state {
        0 => select!(c, 1, 10),
        1 => select!(c, 2, 3),
        2 => select!(c, 4, 11),
        3 => select!(c, 9, 3),
        4 => select!(c, 5, 4),
        5 => select!(c, 6, 4),
        6 => select!(c, 7, 4),
        7 => select!(c, 10, 11),
        _ => unreachable!("Should not be called ever"),
      };
      self.position += 1;
      if state > 9 {
        break;
      }
    }

    match state {
      7 | 9 | 11 => {
        self.commit_token(
          |s| match Tokeniser::parse_string(s.trim_matches('\"').to_string()) {
            Ok(s) => TokenValue::StringLiteral(s),
            Err(e) => TokenValue::Error(s, e),
          },
        )
      }
      10 => self.commit_token(|s| TokenValue::Error(s, ErrorCode::BrokenStringLiteral)),
      _ => self.commit_token(|s| TokenValue::Error(s, ErrorCode::UnterminatedStringLiteral)),
    }
  }

  fn restore(self: &mut Self) -> Token {
    self.start_token();

    while let Some(c) = self.get_char(self.position) {
      if c.is_whitespace() || Tokeniser::is_punctuation(c) {
        break;
      }
      self.position += 1;
    }

    self.commit_token(|s| TokenValue::Error(s, ErrorCode::UnexpectedToken))
  }

  pub fn consume_bracket(self: &mut Self) -> Token {
    self.start_token();

    self.position += 1;

    self.commit_token(|s| match s.chars().nth(0) {
      Some(c) if Tokeniser::is_bracket(c) => TokenValue::Bracket(c),
      _ => TokenValue::Error(s, ErrorCode::UnexpectedToken),
    })
  }

  pub fn consume_operator(self: &mut Self) -> Token {
    self.start_token();

    let mut trie: &Trie = &OP_LIST;

    while let Some(c) = self.get_char(self.position) {
      trie = match trie.get(c) {
        Some(trie) => trie,
        None => {
          break;
        }
      };
      self.position += 1;
    }

    if trie.is_leaf() {
      self.commit_token(|s| TokenValue::Operator(s))
    } else {
      self.commit_token(|s| TokenValue::Error(s, ErrorCode::UnexpectedToken))
    }
  }

  pub fn consume_number_or_dot(self: &mut Self) -> Token {
    self.start_token();

    let mut state: u8 = 0;
    let mut error = false;

    while let Some(c) = self.get_char(self.position) {
      state = match state {
        // 0 - begin
        0 => match c {
          '0' => 1,
          '1'..='9' => 2,
          '.' => 3,
          _ => unreachable!(),
        },
        // 1 - read '0'
        1 => match c {
          '0'..='7' => 4,
          '.' => 5,
          'x' => 6,
          c if Tokeniser::is_punctuation_or_whitespace(c) => {
            break;
          }
          _ => panic!(),
        },
        // 2 - reading decimal
        2 => match c {
          '0'..='9' => 2,
          '.' => 5,
          c if Tokeniser::is_punctuation_or_whitespace(c) => {
            break;
          }
          _ => {
            error = true;
            break;
          }
        },
        // 3 - read '.'
        3 => match c {
          '0'..='9' => 5,
          _ => {
            break;
          }
        },
        // 4 - reading octal
        4 => match c {
          '0'..='7' => 4,
          c if Tokeniser::is_punctuation_or_whitespace(c) => {
            break;
          }
          _ => {
            error = true;
            break;
          }
        },
        // 5 - reading float
        5 => match c {
          '0'..='9' => 5,
          c if Tokeniser::is_punctuation_or_whitespace(c) => {
            break;
          }
          _ => {
            error = true;
            break;
          }
        },
        // 6 - read "0x"
        6 => match c {
          '0' => 7,
          c if c.is_ascii_hexdigit() => 8,
          _ => {
            error = true;
            break;
          }
        },
        // 7 - read "0x0"
        7 => {
          if Tokeniser::is_punctuation_or_whitespace(c) {
            break;
          } else {
            error = true;
            break;
          }
        }
        // 8 - reading hex
        8 => match c {
          c if c.is_ascii_hexdigit() => 8,
          c if Tokeniser::is_punctuation_or_whitespace(c) => {
            break;
          }
          _ => {
            error = true;
            break;
          }
        },
        _ => {
          error = true;
          break;
        }
      };

      self.position += 1;
    }

    if error {
      self.position = self.current_token_start;
      return self.restore();
    }

    match state {
      1 | 7 => self.commit_token(|_| TokenValue::IntLiteral(0)),
      2 => self.parse_int(10),
      3 => self.commit_token(|_| TokenValue::Operator(".".to_string())),
      4 => self.parse_int(8),
      5 => self.parse_float(),
      8 => self.parse_int(16),
      _ => self.commit_token(|s| TokenValue::Error(s, ErrorCode::UnexpectedToken)),
    }
  }

  pub fn parse(self: &mut Self) -> Vec<Token> {
    let mut tokens = Vec::new();

    while let Some(c) = self.get_char(self.position) {
      let to_push = if c.is_alphabetic() {
        self.consume_identifier()
      } else if c == '\'' {
        self.consume_char_literal()
      } else if c == '"' {
        self.consume_string_literal()
      } else if c.is_whitespace() || c == '\n' {
        self.position += 1;
        continue;
      } else if Tokeniser::is_bracket(c) {
        self.consume_bracket()
      } else if c == '.' || c.is_ascii_hexdigit() {
        self.consume_number_or_dot()
      } else if Tokeniser::is_punctuation(c) {
        self.consume_operator()
      } else {
        self.restore()
      };

      tokens.push(to_push);
    }

    tokens
  }
}

pub fn tokenise(code: &str) -> Vec<Token> {
  let source_code = SourceCode::new(code.to_string());
  let mut tokeniser = Tokeniser::new(source_code);

  tokeniser.parse()
}
