use crate::source_code::SourceCode;

use super::{
  check, parsers,
  token::{ErrorCode, TokenValue},
  trie::Trie,
  Token,
};

pub struct Tokeniser<'a> {
  source_code: &'a SourceCode,
  position: usize,
  current_token_start: usize,
}

impl<'a> Tokeniser<'a> {
  pub fn new(source_code: &SourceCode) -> Tokeniser {
    Tokeniser {
      source_code,
      position: 0,
      current_token_start: 0,
    }
  }

  fn start_token(&mut self) {
    self.current_token_start = self.position;
  }

  fn get_char(&self, position: usize) -> Option<char> {
    self.source_code.code.chars().nth(position)
  }

  fn commit_token<F>(&mut self, value: F) -> Token
  where
    F: FnOnce(String) -> TokenValue,
  {
    let code = &self.source_code.code;

    macro_rules! nth_indices {
      ($n: expr) => {
        match code.char_indices().nth($n) {
          Some((c, _)) => c,
          None => code.len(),
        }
      };
    }

    let from = nth_indices!(self.current_token_start);
    let to = nth_indices!(self.position);

    let start = self.source_code.to_location(self.current_token_start).unwrap();
    let end = self.source_code.to_location(self.position).unwrap();

    let value = value(code[from..to].to_string());
    Token::new(value, start, end)
  }

  fn consume_identifier(&mut self) -> Token {
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

  fn consume_char_literal(&mut self) -> Token {
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

    self.commit_token(|s| match closed {
      false => TokenValue::Error(s, ErrorCode::UnterminatedCharLiteral),
      true => match s.chars().count() {
        0 | 1 => unreachable!("Cannot happen"),
        2 => TokenValue::Error(s, ErrorCode::EmptyCharLiteral),
        3 => TokenValue::CharLiteral(s.chars().nth(1).unwrap()),
        len => match parsers::parse_symbol_in_string(&s, 1, true) {
          Err(e) => TokenValue::Error(s, e),
          Ok((c, next)) => {
            if next + 1 < len {
              TokenValue::Error(s, ErrorCode::CharLiteralTooLong)
            } else {
              TokenValue::CharLiteral(c)
            }
          }
        },
      },
    })
  }

  fn consume_string_literal(&mut self) -> Token {
    macro_rules! select {
      ($ch: expr, $success: literal, $failure: literal) => {
        if $ch == '"' {
          $success
        } else {
          $failure
        }
      };
    }

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
    self.commit_token(|s: String| match state {
      7 | 9 | 11 => match parsers::parse_string(s.trim_matches('\"').to_string()) {
        Ok(s) => TokenValue::StringLiteral(s),
        Err(e) => TokenValue::Error(s, e),
      },
      10 => TokenValue::Error(s, ErrorCode::BrokenStringLiteral),
      _ => TokenValue::Error(s, ErrorCode::UnterminatedStringLiteral),
    })
  }

  fn restore(&mut self) -> Token {
    self.start_token();

    while let Some(c) = self.get_char(self.position) {
      if c.is_whitespace() || check::is_punctuation(c) {
        break;
      }
      self.position += 1;
    }

    self.commit_token(|s| TokenValue::Error(s, ErrorCode::UnexpectedToken))
  }

  fn consume_bracket(&mut self) -> Token {
    self.start_token();

    self.position += 1;

    self.commit_token(|s| match s.chars().nth(0) {
      Some(c) if check::is_bracket(c) => TokenValue::Bracket(c),
      _ => TokenValue::Error(s, ErrorCode::UnexpectedToken),
    })
  }

  fn consume_operator(&mut self) -> Token {
    self.start_token();

    let mut trie: &Trie = &check::OP_LIST;

    while let Some(c) = self.get_char(self.position) {
      trie = match trie.get(c) {
        Some(trie) => trie,
        None => {
          break;
        }
      };
      self.position += 1;
    }

    self.commit_token(|s| {
      if trie.is_leaf() {
        TokenValue::Operator(s)
      } else {
        TokenValue::Error(s, ErrorCode::UnexpectedToken)
      }
    })
  }

  fn consume_number_or_dot(&mut self) -> Token {
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
          c if check::is_punctuation_or_whitespace(c) => {
            break;
          }
          _ => panic!(),
        },
        // 2 - reading decimal
        2 => match c {
          '0'..='9' => 2,
          '.' => 5,
          c if check::is_bracket(c) || check::is_punctuation_or_whitespace(c) => {
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
          c if check::is_punctuation_or_whitespace(c) => {
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
          c if check::is_punctuation_or_whitespace(c) => {
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
          if check::is_punctuation_or_whitespace(c) {
            break;
          } else {
            error = true;
            break;
          }
        }
        // 8 - reading hex
        8 => match c {
          c if c.is_ascii_hexdigit() => 8,
          c if check::is_punctuation_or_whitespace(c) => {
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

    self.commit_token(match state {
      1 | 7 => |_| TokenValue::IntLiteral(0),
      2 => |s| parsers::parse_int(s, 10),
      3 => |_| TokenValue::Operator(".".to_string()),
      4 => |s| parsers::parse_int(s, 8),
      5 => parsers::parse_float,
      8 => |s| parsers::parse_int(s, 16),
      _ => |s| TokenValue::Error(s, ErrorCode::UnexpectedToken),
    })
  }

  pub fn parse(&mut self) -> Vec<Token> {
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
      } else if check::is_bracket(c) {
        self.consume_bracket()
      } else if c == '.' || c.is_ascii_hexdigit() {
        self.consume_number_or_dot()
      } else if check::is_punctuation(c) {
        self.consume_operator()
      } else {
        self.restore()
      };

      tokens.push(to_push);
    }

    tokens
  }
}
