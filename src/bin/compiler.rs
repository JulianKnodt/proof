extern crate proof;

use std::env;
use std::io;
use std::fs::File;
use std::io::prelude::*;
use proof::compile::compile::compile;

fn main() {
  let mut out = io::stdout();
  env::args().for_each(|arg| {
    let mut file = File::open(arg).expect("File could not be opened");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("File could not be read");
    let parts = proof::lisp_parse::parse(contents);
    compile(parts.into_iter().next().unwrap(), &mut out).expect("Could not compile");
  });
}

