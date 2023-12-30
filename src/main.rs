#![allow(dead_code)]

mod ast;
mod source_code;
mod tokeniser;

fn main() {
  let code = "42+58+-24";

  println!("{}", code);

  let tokens = tokeniser::from_string(code);
  let ast = ast::from_tokens(tokens);

  println!("{}", ast);
}
