extern crate proof;
use std::io::{self, Read};

fn main() {
  // get input str
  let mut buffer = String::new();
  io::stdin().read_to_string(&mut buffer).unwrap();

  // tokenize input str and then turn into ast
  // then evaluate
  println!("{:?}", proof::lisp_parse::parse(buffer).to_ast().eval(proof::ast::Env::default()));
}
