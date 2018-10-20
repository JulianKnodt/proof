use lisp_parse::{Token};
use ast::{Expr, Type, Defn};
use std::sync::Arc;

impl Token {
  pub fn to_ast(&self) -> Expr {
    match self {
      Token::Word(s) => match &s[..] {
        "let" => panic!("Cannot use reserved keyword let"),
        "defn" => panic!("Cannot use reserved keyword defn"),
        "match" => panic!("Cannot use reserved keyword match"),
        "true" => Expr::Literal(Type::Bool(true)),
        "false" => Expr::Literal(Type::Bool(false)),
        s if s.parse::<i32>().is_ok() => Expr::Literal(Type::Int(s.parse::<i32>().unwrap())),
        s if s.parse::<f32>().is_ok() => Expr::Literal(Type::Float(s.parse::<f32>().unwrap())),
        s if s.starts_with("(") && s.ends_with(")") => Expr::Literal(Type::Str(s.to_string())),
        s => Expr::Variable(s.to_string()),
      },
      Token::Group(tokens) => match tokens.len() {
        0 => Expr::Literal(Type::Unit),
        _ => if let Token::Word(ref spec) = tokens[0] {
          let operands = tokens.iter().skip(1).map(|op| op.to_ast()).collect();
          match &spec[..] {
            "let" => construct_let(operands).expect("Invalid let statement"),
            "defn" => construct_defn_prefix(operands).expect("Invalid defn statement"),
            "match" => unimplemented!("Will implement match later whatever"),
            e => Expr::Call(Box::new(Expr::Variable(e.to_string())), operands),
          }
        } else {
          let operands = tokens.iter().skip(1).map(|op| op.to_ast()).collect();
          Expr::Call(Box::new(tokens[0].to_ast()), operands)
        },
      }
    }
  }
}

fn construct_let(mut operands: Vec<Expr>) -> Option<Expr> {
  if operands.len() != 3 {
    return None;
  }
  let name = if let Expr::Variable(ref name) = operands[0] {
    name.to_string()
  } else {
    return None;
  };
  let into = Box::new(operands.pop().unwrap());
  let value_of = Box::new(operands.pop().unwrap());
  Some(Expr::Assign(name, value_of, into))
}

fn construct_defn_prefix(mut operands: Vec<Expr>) -> Option<Expr> {
  let body = Box::new(if let Some(last) = operands.pop() {
    last
  } else {
    return None
  });

  let mut params = Vec::new();
  for param in operands.iter().skip(1) {
    match param {
      Expr::Variable(name) => params.push(name.to_string()),
      _ => return None,
    }
  }
  if let Expr::Variable(ref name) = operands[0] {
    return Some(Expr::Defn(Defn{
      name: name.to_string(),
      params: params.to_vec(),
      body,
    }))
  }
  None
}
