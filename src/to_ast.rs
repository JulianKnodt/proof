use lisp_parse::{Token};
use eval2::{Expr, Type, Defn};
use std::sync::Arc;

impl Token {
  pub fn to_ast(&self) -> Expr {
    match self {
      Token::Word(s) => match &s[..] {
        "let" | "defn" | "if" => panic!("Reserved keyword used"),
        s if s.parse::<f32>().is_ok() =>
          Expr::Value(Arc::new(Type::Number(s.parse::<f32>().unwrap()))),
        s if s.starts_with("\"") && s.ends_with("\"") =>
          Expr::Value(Arc::new(Type::Str(s.to_string()))),
        s => Expr::Variable(s.to_string()),
      },
      Token::Group(ref g) if g.len() == 0 => Expr::Value(Arc::new(Type::Unit)),
      Token::Group(ref g) => if let Token::Word(ref s) = g[0] {
        match &s[..] {
          "let" => unimplemented!(),
          "defn" => unimplemented!(),
          "if" => unimplemented!(),
          func => unimplemented!(),
        }
      } else {
        // the case where the first element is a group
        unimplemented!();
      }
    }
  }
}
