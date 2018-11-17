extern crate proof;
use std::io::Write;
use std::io::{self, stdout};

fn main() {
  // tokenize input str and then turn into ast
  // then evaluate
//  println!("{:?}", proof::lisp_parse::parse(buffer).to_ast().eval(proof::ast::Env::default()));
  let mut buffer = String::new();
  let mut global_env = proof::ast::Env::default_global();
  loop {
    print!(">> ");
    stdout().flush().expect("Could not flush to stdout, strange.");
    match io::stdin().read_line(&mut buffer) {
      Ok(0) => break,
      Ok(_) => if has_matching_parens(&buffer) {
        proof::lisp_parse::parse(buffer)
          .iter()
          .for_each(|tokenized| {
            print!("= {:?}",
              tokenized.to_ast().eval(proof::ast::Env::default(), &mut global_env));
          });
        buffer = String::new();
      },
      Err(e) => {
        println!("{}, exitting...", e);
        break;
      },
    }
    println!("");
  }
  println!("\nFac ut vivas!");
}

// this was my google internship question today lol
fn has_matching_parens(s: &String) -> bool {
  let mut count = 0;
  for c in s.chars() {
    match c {
      '(' => count = count + 1,
      ')' => {
        count = count - 1;
        if count < 0 {
          return false
        }
      }
      _ => (),
    }
  }
  if count == 0 {
    true
  } else {
    println!("        ----------------- Missing {} ) parens", count);
    false
  }
}
