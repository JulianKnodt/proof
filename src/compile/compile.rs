use std::io::prelude::*;
use std::io;

use lisp_parse::Token;

use std::collections::HashMap;

mod builtins {
  macro_rules! builtin_fn {
    ($builtin_name: ident, $content: tt) => (
      pub fn $builtin_name(w: &mut Write) -> io::Result<()> {
        write!(w, $content)
      }
    );

    ($builtin_name: ident, $content: tt, $($format_args:expr),*) => (
      pub fn $builtin_name(w: &mut Write) -> io::Result<()> {
        write!(w, $content, $($format_args),*)
      }
    );
  }

  use std::io;
  use super::*;
  builtin_fn!(fxadd1, "addl ${}, %eax\n", Immed::Fixnum(1).value());
  builtin_fn!(fxsub1, "subl ${}, %eax\n", Immed::Fixnum(1).value());
  pub fn char_to_fixnum(w: &mut Write) -> io::Result<()> {
    write!(w, "xor ${}, %eax
      shr ${}, %eax\n", CHAR_TAG, CHAR_SHIFT - FX_SHIFT)
  }
  pub fn fixnum_to_char(w: &mut Write) -> io::Result<()> {
    write!(w, "shl ${}, %eax
      orl ${}, %eax\n", CHAR_SHIFT - FX_SHIFT, CHAR_TAG)
  }
  pub fn is_fxzero(w: &mut Write) -> io::Result<()> {
    write!(w,
      "cmp ${}, %eax
      mov ${}, %eax
      sete %al
      shl ${}, %eax
      orl ${}, %eax
      ", Immed::Fixnum(0).value(), 0, 6, Immed::Bool(false).value())
  }
  builtin_fn!(is_null,
      "cmp ${}, %eax
      mov ${}, %eax
      sete %al
      shl ${}, %eax
      orl ${}, %eax
      ", Immed::Nil.value(), 0, 6, Immed::Bool(false).value());

  pub fn not(w: &mut Write) -> io::Result<()> {
    write!(w,
      "cmp ${}, %eax
      mov ${}, %eax
      sete %al
      shl ${}, %eax
      orl ${}, %eax
      ", FALSE, 0, 6, Immed::Bool(false).value())
  }
  pub fn is_fixnum(w: &mut Write) -> io::Result<()> {
    write!(w,
      "and ${}, %eax
      setz %al
      shl $6, %eax
      orl ${}, %eax
      ", FX_MASK, Immed::Bool(false).value())
  }
//  pub fn is_boolean
}

macro_rules! with_items {
  ($hashmap: expr) => ($hashmap);
  ($hashmap: expr, $($item: expr),*,) => (
    with_items!($hashmap, $($item),*)
  );
  ($hashmap: expr, $($item: expr),*) => (
    {
     $(
      let (name, num_args, func) = $item;
      $hashmap.insert((name, num_args), func as fn(&mut Write) -> io::Result<()>);
     )*
    }
  );
}
lazy_static!{
  static ref Builtin: HashMap<(&'static str, usize), fn(&mut Write) -> io::Result<()>> = {
    let mut result = HashMap::new();
    with_items!(result,
      ("fixnum?", 1, builtins::is_fixnum),
      ("not", 1, builtins::not),
      ("fxadd1", 1, builtins::fxadd1),
      ("fxsub1", 1, builtins::fxsub1),
      ("fxzero?", 1, builtins::is_fxzero),
      ("null?", 1, builtins::is_null),
      ("char->fixnum", 1, builtins::char_to_fixnum),
      ("fixnum->char", 1, builtins::fixnum_to_char),
    );
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
        _ => unimplemented!(), // This is the case where the first arg evals to fn
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

const CHAR_SHIFT: i32 = 8;
const CHAR_TAG : i32 = 15;
const FX_SHIFT : i32 = 2;
const FX_MASK : i32 = 0x03; // just 3 lol
const FALSE : i32 = 0b00101111;
const TRUE : i32 = 0b01101111;
const NIL : i32 = 0b00111111;

impl Immed {
  fn value(&self) -> i32 {
    match self {
      Immed::Fixnum(u) =>  u << FX_SHIFT,
      Immed::Bool(b) => if *b { TRUE } else { FALSE },
      Immed::Char(c) => (*c as i32) << CHAR_SHIFT | CHAR_TAG,
      Immed::Nil => NIL,
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


