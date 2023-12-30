#![allow(dead_code)]

use std::io::{stdin, stdout, Write};

use computer::Computer;

mod ast;
mod computer;
mod source_code;
mod tokeniser;

fn main() {
  println!("Enter the expression (l or list to list all variables, q or quit to exit)");

  let mut computer = Computer::new();

  loop {
    print!("> ");
    stdout().flush().unwrap();

    let mut code = String::new();
    let res = stdin().read_line(&mut code).expect("Cannot read from stdin, exiting");

    if res == 0 {
      break;
    }

    let code = code.trim();

    if code.len() < 1 {
      continue;
    } else if code == "q" || code == "quit" {
      break;
    } else if code == "l" || code == "list" {
      for (k, v) in computer.iter() {
        println!("{} = {}", k, v)
      }
      continue;
    }

    let ast = ast::from_string(code);
    let result = computer.compute(&ast);

    match result {
      Ok(result) => println!("{}", result),
      Err(e) => println!("Error: {}", e),
    };
  }
}
