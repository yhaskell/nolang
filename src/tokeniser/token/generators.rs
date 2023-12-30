/// Returns Error token
///
/// # Examples
///
/// ```
/// let token = error!("1failed%_token", ErrorCode::UnexpectedToken)
/// ```
macro_rules! error {
  ($v: expr, $t: expr) => {
    crate::tokeniser::token::TokenValue::Error($v.to_string(), $t)
  };
}

/// Returns Identifier token
///
/// # Examples
/// ```
/// let token = ident!("test")
/// ```
macro_rules! ident {
  ($value: expr) => {
    crate::tokeniser::token::TokenValue::Identifier($value.to_string())
  };
}

/// Returns Bracket token
///
/// # Examples
/// ```
/// let token = bracket!('(')
/// ```
macro_rules! bracket {
  ($value: expr) => {
    crate::tokeniser::token::TokenValue::Bracket($value)
  };
}

/// Returns Char Literal token
///
/// # Examples
/// ```
/// let token = char!('ü')
/// ```
macro_rules! char {
  ($c: expr) => {
    crate::tokeniser::token::TokenValue::CharLiteral($c)
  };
}

/// Returns Operator token
///
/// # Examples
/// ```
/// let token = operator!("::")
/// ```
macro_rules! operator {
  ($value: expr) => {
    crate::tokeniser::token::TokenValue::Operator($value.to_string())
  };
}

/// Returns String Literal token
///
/// # Examples
/// ```
/// let token = string!("test")
/// ```
macro_rules! string {
  ($c: expr) => {
    crate::tokeniser::token::TokenValue::StringLiteral($c.to_string())
  };
}

/// Returns Int Literal token
///
/// # Examples
/// ```
/// let token = int!(174)
/// ```
macro_rules! int {
  ($c: expr) => {
    crate::tokeniser::token::TokenValue::IntLiteral($c)
  };
}

/// Returns Float Literal token
///
/// # Examples
/// ```
/// let token = float!(7.45)
/// ```
macro_rules! float {
  ($c: expr) => {
    crate::tokeniser::token::TokenValue::FloatLiteral($c)
  };
}

pub(crate) use bracket;
pub(crate) use char;
pub(crate) use error;
pub(crate) use float;
pub(crate) use ident;
pub(crate) use int;
pub(crate) use operator;
pub(crate) use string;
