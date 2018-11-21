use std::io::prelude::*;
use std::io;

use lisp_parse::Token;

enum Sexp {
  Immed(Immed),
  Malformed(String),
}

impl Sexp {
  pub fn type_of(s: &str) -> Self {
    match s.as_ref() {
      "#t" => Sexp::Immed(Immed::Bool(true)),
      "#f" => Sexp::Immed(Immed::Bool(false)),
      "nil" => Sexp::Immed(Immed::Nil),
      "()" => Sexp::Immed(Immed::Nil),
      _ if s.trim().parse::<i32>().is_ok() =>
        Sexp::Immed(Immed::Fixnum(s.trim().parse::<i32>().unwrap())),
      _ if s.len() == 3 && s.starts_with("#\\") =>
        Sexp::Immed(Immed::Char(s.bytes().last().unwrap())),
      _ => Sexp::Malformed(s.to_string()),
    }
  }
}


enum Immed {
  Fixnum(i32),
  Bool(bool),
  Char(u8),
  Nil,
}

impl Immed {
  fn write_to(self, w: &mut Write) -> io::Result<()> {
    write!(w, "mov ${:#b}, %eax\n\tret\n",
    match self {
      Immed::Fixnum(u) =>  u << 2,
      Immed::Bool(b) => if b { 0b01101111 } else { 0b00101111 },
      Immed::Char(c) => (c as i32) << 8 | 15,
      Immed::Nil => 0b00111111,
    })
  }
}

fn prelude(w: &mut Write) -> io::Result<()> {
  write!(w, "
    .text
    .globl _scheme
  _scheme: ## @_scheme
  ")
}


pub fn compile(body: Token, to: &mut Write) -> io::Result<()> {
  prelude(to)?;
  match body {
    Token::Word(s) => match Sexp::type_of(s.as_ref()) {
      Sexp::Immed(i) => i.write_to(to),
      Sexp::Malformed(m) => panic!("Error while compiling {}", m),
    },
    Token::Group(_tokens) => {
      unimplemented!()
    },
  }
}


