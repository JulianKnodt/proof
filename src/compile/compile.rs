use std::io::prelude::*;
use std::io;

use lisp_parse::Token;

use std::collections::HashMap;

mod builtins {
  use std::io;
  use super::*;
  pub fn fxadd1(w: &mut Write) -> io::Result<()> {
    write!(w, "addl ${}, %eax\n", Immed::Fixnum(1).value())
  }
}

lazy_static!{
  static ref Builtin: HashMap<(&'static str, usize), fn(&mut Write) -> io::Result<()>> = {
    let mut result = HashMap::new();
    // first is the name of the function, and the second is the number of arguments
    result.insert(("fxadd1", 1), builtins::fxadd1 as fn(&mut Write) -> io::Result<()>);
    result
  };
}

enum Sexp {
  Immed(Immed),
  Expr(String, Vec<Sexp>),
  Malformed(String),
}

impl Sexp {
  fn type_of(t: &Token) -> Self {
    match t {
      Token::Word(s) => match s.as_ref() {
        "#t" => Sexp::Immed(Immed::Bool(true)),
        "#f" => Sexp::Immed(Immed::Bool(false)),
        "nil" => Sexp::Immed(Immed::Nil),
        "()" => Sexp::Immed(Immed::Nil),
        _ if s.trim().parse::<i32>().is_ok() =>
          Sexp::Immed(Immed::Fixnum(s.trim().parse::<i32>().unwrap())),
        _ if s.len() == 3 && s.starts_with("#\\") =>
          Sexp::Immed(Immed::Char(s.bytes().last().unwrap())),
        _ => Sexp::Malformed(s.to_string()),
      },
      Token::Group(g) => match g.as_slice() {
        [] => Sexp::Immed(Immed::Nil),
        [Token::Word(fn_name), args..] =>
          Sexp::Expr(fn_name.to_string(), args.iter().map(|arg| Sexp::type_of(arg)).collect()),
        _ => unimplemented!(),
      },
    }
  }
  fn emit(&self, w: &mut Write) -> io::Result<()> {
    match self {
      Sexp::Immed(v) => write!(w, "mov ${:#b}, %eax\n", v.value()),
      Sexp::Expr(fn_name, args) => {
        for arg in args {
          arg.emit(w)?;
        }
        match Builtin.get(&(fn_name.as_ref(), args.len())) {
          Some(func) => func(w),
          None => panic!("No such function {}", fn_name),
        }
      },
      Sexp::Malformed(_) => panic!("Emit called on malformed"),
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
  fn value(&self) -> i32 {
    match self {
      Immed::Fixnum(u) =>  u << 2,
      Immed::Bool(b) => if *b { 0b01101111 } else { 0b00101111 },
      Immed::Char(c) => (*c as i32) << 8 | 15,
      Immed::Nil => 0b00111111,
    }
  }
}

fn prelude(w: &mut Write) -> io::Result<()> {
  write!(w, "
    .text
    .globl _scheme
  _scheme: ## @_scheme
  ")
}


pub fn compile(body: &Token, to: &mut Write) -> io::Result<()> {
  prelude(to)?;
  Sexp::type_of(body).emit(to)?;
  write!(to, "  ret")
}


