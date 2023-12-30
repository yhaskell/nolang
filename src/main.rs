#![allow(dead_code)]

use std::io::{stdin, stdout, Write};

use computer::compute;

mod ast;
mod computer;
mod source_code;
mod tokeniser;

fn main() {
  println!("Enter the expression (q to exit)");

  loop {
    print!("> ");
    stdout().flush().unwrap();

    let mut code = String::new();
    stdin().read_line(&mut code).expect("Cannot read from stdin, exiting");

    let code = code.trim();

    if code == "q" {
      return;
    }

    let ast = ast::from_string(code);
    let result = compute(&ast);

    match result {
      Ok(result) => println!("{} = {}", code, result),
      Err(e) => println!("Error: {}", e),
    };
  }
}
