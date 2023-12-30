use super::{from_string, token::generators::*, token::ErrorCode};

macro_rules! test {
  ( $name: ident, $code: expr, { $($index: expr => $token: expr),* } ) => {
    #[test]
    fn $name() {
      let tokens = from_string($code);

      $(
      assert_eq!(tokens[$index].value, $token);
      )*
    }
  };
  ( $name: ident, $code: expr, $len: expr, { $($index: expr => $token: expr),* } ) => {
    #[test]
    fn $name() {
      let tokens = from_string($code);

      assert_eq!($len, tokens.len());

      $(
      assert_eq!(tokens[$index].value, $token);
      )*
    }
  };
}

test!(identifier_single, "test", { 0 => ident!("test") });
test!(identifier_multiple, "test success", 2, { 0 => ident!("test"), 1 => ident!("success") });
test!(identifier_with_error, "1test", { 0 => error!("1test", ErrorCode::UnexpectedToken) });

test!(char_single, "'c'", { 0 => char!('c') });
test!(char_multiple, r"'c' '\n' '\u0041'", 3, { 0 => char!('c'), 1 => char!('\n'), 2 => char!('A') });
test!(char_with_escape_sequence, r"'\n'", { 0 => char!('\n') });
test!(char_with_wrong_escape_sequence, r"'\a'", { 0 => error!(r"'\a'", ErrorCode::UnknownEscapeSequence) });
test!(char_with_unicode_symbol, "'試' 'é'", { 0 => char!('試'), 1 => char!('é') });
test!(char_with_unicode_escape_sequence, r"'\u0041'", { 0 => char!('A') });

test!(bracket_single, "(", { 0 => bracket!('(') });
test!(bracket_multiple, "()", 2,  { 0 => bracket!('('), 1 => bracket!(')') });

test!(string_single, r#""test""#, { 0 => string!("test") });
test!(string_unfinished_with_eof, r#""test"#, { 0 => error!("\"test", ErrorCode::UnterminatedStringLiteral) });
test!(string_unfinished_with_newline, "\"test\n\"test\"", 2, { 0 => error!("\"test", ErrorCode::UnterminatedStringLiteral), 1 => string!("test") });
test!(string_multiple, &r#""""test test""""#.replace(' ', "\n"), { 0 => string!("test\ntest") });
test!(string_with_escape_sequences, r#""\n\t\u0041""#, { 0 => string!("\n\tA") });

test!(int_decimal_zero, "0", { 0 => int!(0) });
test!(int_decimal_single, "42", { 0 => int!(42) });
test!(int_very_long, "340282366920938463463374607431768211455", { 0 => int!(340282366920938463463374607431768211455) });
test!(int_too_long_error, "340282366920938463463374607431768211456", { 0 => error!("340282366920938463463374607431768211456", ErrorCode::IntLiteralTooLong) });
test!(int_octal, "0100", { 0 => int!(64) });
test!(int_hex_zero, "0x0", { 0 => int!(0) });
test!(int_hex, "0x7f", { 0 => int!(127) });
test!(int_multiple, "5 7", {0 =>  int!(5), 1 => int!(7) });
test!(int_divided_by_operators, "5+7", { 0 => int!(5), 2 => int!(7) });

test!(float_single, "7123.5514", { 0 => float!(7123.5514) });
test!(float_without_integer_part, ".45", { 0 => float!(0.45) });
test!(float_without_floating_part, "42.", { 0 => float!(42.0) });

test!(operator_dot, ".", { 0 => operator!(".") });
test!(operator_single_char, "%", { 0 => operator!("%")});
test!(operator_multiple_chars, "::", { 0 => operator!("::") });
test!(operator_multiple, "++::--", { 0 => operator!("++"), 1 => operator!("::"), 2 => operator!("--") });
test!(combined, "++test * (a-b).c", {
  0 => operator!("++"),
  1 => ident!("test"),
  2 => operator!("*"),
  3 => bracket!('('),
  4 => ident!("a"),
  5 => operator!("-"),
  6 => ident!("b"),
  7 => bracket!(')'),
  8 => operator!("."),
  9 => ident!("c")
});
