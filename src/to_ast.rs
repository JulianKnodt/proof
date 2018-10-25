use lisp_parse::{Token};
use ast::{Expr, Type, Defn, ParamType};
use std::sync::Arc;

impl Token {
  pub fn to_ast(&self) -> Expr {
    println!("{:?}", self);
    match self {
      Token::Word(s) => match &s[..] {
        "let" | "defn" | "if" => panic!("Reserved keyword used"),
        "[]" => Expr::Value(Type::new_empty_list()),
        s if s.parse::<f32>().is_ok() =>
          Expr::Value(Type::new_number(s.parse::<f32>().unwrap())),
        s if s.starts_with("\"") && s.ends_with("\"") =>
          Expr::Value(Arc::new(Type::Str(s.to_string()))),
        s => Expr::Variable(s.to_string()),
      },
      Token::Group(ref g) if g.len() == 0 => Expr::Value(Type::unit()),
      Token::Group(ref g) => if let Token::Word(ref s) = g[0] {
        match &s[..] {
          "let" => match g.len() {
            4 => {
              let bound_to = if let Some(Token::Word(s)) = g.get(1) { s }
                else { panic!("Must assign to name") };
              Expr::Assign(bound_to.to_string(), Arc::new(g[2].to_ast()), Arc::new(g[3].to_ast()))
            },
            3 => {
              // TODO global let
              unimplemented!()
            },
            _ => panic!("Invalid let statement, must have 2-3 operands"),
          },
          "defn" => {
            let name = (if let Some(Token::Word(s)) = g.get(1) { s }
              else { panic!("Must assign to name") }).to_string();
            let body = Arc::new(g.last().expect("Defn must have body").to_ast());
            Expr::Defn(Arc::new(Defn{
              name,
              params: g[0..(g.len()-1)].iter().skip(1).map(|it| match it {
                Token::Word(s) if s.starts_with("&") => ParamType::Rest(s[1..].to_string()),
                Token::Word(s) => ParamType::Singular(s.to_string()),
                Token::Group(_) => panic!("Can only have string params"),
              }).collect(),
              body,
            }))
          },
          "if" => Expr::If(Arc::new(g[0].to_ast()),
          Arc::new(g[1].to_ast()), Arc::new(g[2].to_ast())),
          func => Expr::Call(Arc::new(Expr::Variable(func.to_string())), g.iter().skip(1)
            .map(|it| Arc::new(it.to_ast())).collect())
        }
      } else {
        // the case where the first element is a group
        unimplemented!();
      }
    }
  }
}
