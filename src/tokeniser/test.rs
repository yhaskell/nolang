macro_rules! assert_token_value {
  ($t: expr, $v: expr) => {
    assert_eq!($t.value, $v)
  };
}

macro_rules! error {
  ($v: expr, $t: expr) => {
    crate::tokeniser::TokenValue::Error($v.to_string(), $t)
  };
}

mod identifier {
  use crate::tokeniser::{token::ErrorCode, tokenise};

  macro_rules! ident {
    ($value: expr) => {
      crate::tokeniser::TokenValue::Identifier($value.to_string())
    };
  }

  #[test]
  fn can_tokenise_one() {
    let tokens = tokenise("test");
    assert_eq!(tokens.len(), 1);

    assert_token_value!(tokens[0], ident!("test"))
  }

  #[test]
  fn can_tokenise_many() {
    let tokens = tokenise("test success");
    assert_eq!(tokens.len(), 2);

    assert_token_value!(tokens[0], ident!("test"));
    assert_token_value!(tokens[1], ident!("success"));
  }

  #[test]
  fn errors_on_id_starting_with_number() {
    let tokens = tokenise("1test");
    assert_eq!(tokens.len(), 1);

    assert_token_value!(tokens[0], error!("1test", ErrorCode::UnexpectedToken))
  }
}

mod char_literal {
  use crate::tokeniser::{
    token::{ErrorCode, TokenValue},
    tokenise,
  };

  macro_rules! char {
    ($c: expr) => {
      TokenValue::CharLiteral($c)
    };
  }

  #[test]
  fn can_tokenise_simple() {
    let tokens = tokenise("'c'");

    assert_eq!(tokens.len(), 1);

    assert_token_value!(tokens[0], char!('c'))
  }

  #[test]
  fn can_tokenise_many() {
    let tokens = tokenise("'c' '\\n' '\\u0041'");
    assert_eq!(tokens.len(), 3);

    assert_token_value!(tokens[0], char!('c'));
    assert_token_value!(tokens[1], char!('\n'));
    assert_token_value!(tokens[2], char!('A'));
  }

  #[test]
  fn can_tokenise_escape() {
    let tokens = tokenise("'\\n'");

    assert_eq!(tokens.len(), 1);

    assert_token_value!(tokens[0], char!('\n'))
  }

  #[test]
  fn errors_when_wrong_escape() {
    let tokens = tokenise("'\\a'");
    assert_token_value!(tokens[0], error!("'\\a'", ErrorCode::UnknownEscapeSequence))
  }

  #[test]
  fn can_tokenise_unicode() {
    let tokens = tokenise("'試' 'é'");

    assert_token_value!(tokens[0], char!('試'));
    assert_token_value!(tokens[1], char!('é'));
  }

  #[test]
  fn can_tokenise_unicode_escape() {
    let tokens = tokenise("'\\u0041'");
    assert_token_value!(tokens[0], char!('A'))
  }
}

mod bracket {
  use crate::tokeniser::tokenise;

  macro_rules! bracket {
    ($value: expr) => {
      crate::tokeniser::TokenValue::Bracket($value)
    };
  }

  #[test]
  fn can_tokenise_one() {
    let tokens = tokenise("(");
    assert_token_value!(tokens[0], bracket!('('))
  }
}

mod string_literal {
  use crate::tokeniser::{
    token::{ErrorCode, TokenValue},
    tokenise,
  };

  macro_rules! string {
    ($c: expr) => {
      TokenValue::StringLiteral($c.to_string())
    };
  }

  #[test]
  fn can_tokenise_simple() {
    let tokens = tokenise("\"test\"");

    assert_token_value!(tokens[0], string!("test"));
  }

  #[test]
  fn errors_on_unfinished_string() {
    let tokens = tokenise("\"test");

    assert_token_value!(
      tokens[0],
      error!("\"test", ErrorCode::UnterminatedStringLiteral)
    );
  }

  #[test]
  fn errors_on_string_newline() {
    let tokens = tokenise("\"test\n\"test\"");

    assert_token_value!(
      tokens[0],
      error!("\"test", ErrorCode::UnterminatedStringLiteral)
    );

    assert_token_value!(tokens[1], string!("test"))
  }

  #[test]
  fn can_tokenise_multiline_string() {
    let tokens = tokenise("\"\"\"test\ntest\"\"\"");

    assert_token_value!(tokens[0], string!("test\ntest"))
  }

  #[test]
  fn can_tokenise_string_with_escape_sequences() {
    let tokens = tokenise("\"\\n\\t\\u0041\"");

    assert_token_value!(tokens[0], string!("\n\tA"))
  }
}

mod number_or_dot {
  use crate::tokeniser::{tokenise, TokenValue};

  macro_rules! int {
    ($c: expr) => {
      TokenValue::IntLiteral($c)
    };
  }
  macro_rules! float {
    ($c: expr) => {
      TokenValue::FloatLiteral($c)
    };
  }

  #[test]
  fn can_consume_0() {
    let tokens = tokenise("0");

    assert_token_value!(tokens[0], int!(0))
  }

  #[test]
  fn can_consume_decimal() {
    let tokens = tokenise("42");

    assert_token_value!(tokens[0], int!(42))
  }

  #[test]
  fn can_consume_very_long() {
    let tokens = tokenise("340282366920938463463374607431768211455");

    assert_token_value!(tokens[0], int!(340282366920938463463374607431768211455))
  }

  #[test]
  fn errors_on_too_large_number() {
    let tokens = tokenise("340282366920938463463374607431768211456");

    assert_token_value!(
      tokens[0],
      error!(
        "340282366920938463463374607431768211456",
        crate::tokeniser::token::ErrorCode::IntLiteralTooLong
      )
    )
  }

  #[test]
  fn can_consume_octal() {
    let tokens = tokenise("0100");
    assert_token_value!(tokens[0], int!(64))
  }

  #[test]
  fn can_consume_hex_0() {
    let tokens = tokenise("0x0");

    assert_token_value!(tokens[0], int!(0))
  }

  #[test]
  fn can_consume_hex() {
    let tokens = tokenise("0x7f");

    assert_token_value!(tokens[0], int!(127));
  }

  #[test]
  fn can_consume_float() {
    let tokens = tokenise("7.5");

    assert_token_value!(tokens[0], float!(7.5));
  }

  #[test]
  fn can_consume_float_with_no_int_part() {
    let tokens = tokenise(".45");

    assert_token_value!(tokens[0], float!(0.45));
  }

  #[test]
  fn can_consume_float_with_no_float_part() {
    let tokens = tokenise("42.");

    assert_token_value!(tokens[0], float!(42.0));
  }

  #[test]
  fn can_consume_dot() {
    let tokens = tokenise(".");

    assert_token_value!(tokens[0], TokenValue::Operator(".".to_string()))
  }

  #[test]
  fn can_consume_multiple() {
    let tokens = tokenise("5 7");

    assert_token_value!(tokens[0], int!(5));
    assert_token_value!(tokens[1], int!(7));
  }

  #[test]
  fn can_consume_numbers_with_operators() {
    let tokens = tokenise("5+7");

    assert_token_value!(tokens[0], int!(5));
    assert_token_value!(tokens[2], int!(7));
  }
}
