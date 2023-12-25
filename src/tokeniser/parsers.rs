use std::str::FromStr;

use super::token::{ErrorCode, TokenValue};

pub fn parse_symbol_in_string(s: &str, pos: usize, parse_unicode: bool) -> Result<(char, usize), ErrorCode> {
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
    match parse_symbol_in_string(&s, pos, true) {
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

pub fn parse_int(s: String, radix: u32) -> TokenValue {
  let src = if radix == 16 { s.trim_start_matches("0x") } else { &s };

  match u128::from_str_radix(src, radix) {
    Ok(n) => TokenValue::IntLiteral(n),
    _ => TokenValue::Error(s, ErrorCode::IntLiteralTooLong),
  }
}

pub fn parse_float(s: String) -> TokenValue {
  match f64::from_str(&s) {
    Ok(n) => TokenValue::FloatLiteral(n),
    _ => TokenValue::Error(s, ErrorCode::FloatLiteralTooLong),
  }
}
