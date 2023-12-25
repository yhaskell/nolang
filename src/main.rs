#![allow(dead_code)]

mod source_code;
mod tokeniser;

fn main() {
  let code = "42++58+-+24+|2";

  println!("{}", code);

  let tokens = tokeniser::tokenise(code);
  for token in tokens {
    println!("{:?}", token.value);
  }
}
