use std::io::prelude::*;
use std::io;

use lisp_parse::Token;

enum Repr {
  Fixnum(i32),
  Bool(bool),
  Char(char),
  Nil,
}

impl Repr {
  fn write_to(self, w: &mut Write) -> io::Result<()> {
    match self {
      Repr::Fixnum(i) => unimplemented!(),
      Repr::Bool(b) => unimplemented!(),
      Repr::Char(c) => unimplemented!(),
      Repr::Nil => unimplemented!(),
    }
  }
}

pub fn compile(body: Token, to: &mut Write) -> io::Result<()> {
  match body {
    Token::Word(s) => match s.as_ref() {
      "t" => Repr::Bool(true).write_to(to),
      "f" => Repr::Bool(false).write_to(to),
      _ if s.trim().parse::<i32>().is_ok() =>
        Repr::Fixnum(s.trim().parse::<i32>().unwrap()).write_to(to),
      _ if s.len() == 3 && s.starts_with("#\\") =>
        Repr::Char(s.chars().last().unwrap()).write_to(to),
      _ => panic!("Did not match on input"),
    },
    Token::Group(tokens) => {
      unimplemented!()
    },
  }
}


