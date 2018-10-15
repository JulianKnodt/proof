use std::collections::HashMap;

#[derive(Clone)]
pub enum Type {
  Free(Box<Expr>),
  // Special cases
  Unit,

  Int(i32),
  Float(f32),
  Bool(bool),
  Str(String),
  Tuple(Box<Type>, Box<Type>),

  Infix(Env, String, String, Box<Expr>, String), //left name fn body right name
  Prefix(Env, String, Vec<String>, Box<Expr>), //Name in body

  List(Box<List>),

  // implement enums here TODO
}

impl Type {
  fn equals(&self, o: &Self) -> bool {
    match self {
      Type::Unit => if let Type::Unit = o { true } else { false },
      Type::Int(a) => if let Type::Int(b) = o { a == b } else { false },
      Type::Float(a) => if let Type::Float(b) = o { a == b } else { false },
      Type::Bool(a) => if let Type::Bool(b) = o { a == b } else { false },
      Type::Str(a) => if let Type::Str(b) = o { a == b } else { false },
      Type::Tuple(a, b) =>
        if let Type::Tuple(c, d) = o { a.equals(c) && b.equals(d) } else { false },
      Type::List(a) => if let Type::List(b) = o { a.equals(b) } else { false },
      Type::Free(a) => if let Type::Free(b) =o { a.equals(b) } else { false },
      _ => false,
    }
  }
}

#[derive(Clone)]
pub enum List {
  End,
  Cons(Type, Box<List>),
}

impl List {
  fn equals(&self, o: &Self) -> bool {
    match self {
      List::End => if let List::End = o { true } else { false },
      List::Cons(hd, tl) => if let List::Cons(h2, t2) = o {
        hd.equals(h2) && tl.equals(t2)
      } else { false },
    }
  }
}

#[derive(Clone)]
pub struct InfixDefn {
  name: String,
  left_name: String,
  right_name: String,
  body: Box<Expr>,
}

#[derive(Clone)]
pub struct PrefixDefn {
  name: String,
  params: Vec<String>,
  body: Box<Expr>,
}

#[derive(Clone)]
pub enum MatchPatterns {
  Cons(String, String),
  EmptyList,
  Tuple(String, String),
  Any(String),
  Ignored,
}

impl MatchPatterns {
  fn matches(&self, o: Type) -> Option<Vec<(String, Type)>> {
    match self {
      MatchPatterns::EmptyList => if let Type::List(i) = o { if let List::End = *i {
        return Some(Vec::new())
      }},
      MatchPatterns::Cons(a, b) => {
        if let Type::List(i) = o {
          let intermediary = *i;
          if let List::Cons(hd, tl) = intermediary {
            return Some(vec!((a.to_string(), hd), (b.to_string(), Type::List(tl))))
          }
        }
      }
      MatchPatterns::Tuple(a, b) => if let Type::Tuple(f, s) = o {
        return Some(vec!((a.to_string(), *f), (b.to_string(), *s)))
      },
      MatchPatterns::Any(name) => return Some(vec!((name.to_string(), o))),
      MatchPatterns::Ignored => return Some(Vec::new()),
    }
    None
  }
}

// TODO optimize by moving fields to structs, so can reduce size of enums.
#[derive(Clone)]
pub enum Expr {
  Literal(Type),
  Variable(String),
  DefnInfix(InfixDefn), // define 1st as infix of 2nd in Exp
  DefnPrefix(PrefixDefn), // define 1st as prefix of 2nd in Exp
  InfixCall(Box<Expr>, Box<Expr>, Box<Expr>), // call infix(2nd) on 1st and 3rd
  PrefixCall(Box<Expr>, Vec<Expr>), // call 1st on rest of Expressions
  Assign(String, Box<Expr>, Box<Expr>), // assign to name value of 1st expression in 2nd
  Match(Box<Expr>, Vec<(MatchPatterns, Expr)>), // compare value of 1st expr to each of 2nd
}

type Env = HashMap<String, Expr>;

pub fn env_with<'a>(env: &Env, name: &String, value: Expr) -> Env {
  let mut out : HashMap<String, Expr> = HashMap::new();
  env.into_iter().for_each(|(k,v)| {
    out.insert(k.to_string(), v.clone());
  });
  out.insert(name.to_string(), value.clone());
  out
}

impl Expr {
  pub fn eval<'a>(&'a self, env: &Env) -> Expr {
    match self {
      Expr::Literal(x) => Expr::Literal(x.clone()),
      Expr::Variable(name) => match env.get(name) {
        Some(sub_expr) => sub_expr.eval(env),
        None => Expr::Literal(Type::Free(Box::new(Expr::Variable(name.to_string())))),
      },
      Expr::Assign(name, value, rest) => rest.eval(&env_with(env, &name, value.eval(env))),
      Expr::PrefixCall(func, args) => {
        let prefix = func.eval(env);
        match prefix.clone() {
          Expr::Literal(Type::Prefix(clos,name,arg_names, body)) =>
            body.eval(&args
              .iter()
              .map(|it| it.eval(env))
              .enumerate()
              .fold(env_with(&clos, &name, prefix),
                |acc, (i, arg)| env_with(&acc, &arg_names[i], arg))),
          _ => panic!("Cannot apply non-prefix func"),
        }
      },
      Expr::InfixCall(l, oper, r) => {
        let infix = oper.eval(env);
        match infix.clone() {
          Expr::Literal(Type::Infix(clos,name,l_name, body, r_name)) => {
            let l_val = l.eval(env);
            let r_val = r.eval(env);
            let with_self = env_with(&clos,&name,infix);
            body.eval(&env_with(&env_with(&with_self, &l_name, l_val), &r_name, r_val))
          },
          _ => panic!("Cannot apply non-infix func as infix operator"),
        }
      },
      Expr::Match(against, branches) => match against.eval(env) {
        Expr::Literal(v) =>
          branches
            .iter()
            .filter_map(move |(branch,next)|
              branch.matches(v.clone()).and_then(|binds| Some((binds, next))))
            .next()
            .map(|(bindings, next)| next.eval(&bindings.iter()
              .fold(env.clone(),
                |acc,(name,val)|env_with(&acc, name, Expr::Literal(val.clone())))))
            .expect("No matching branch"),
        _ => panic!("Cannot match against non-literal"),
      },
      Expr::DefnInfix(InfixDefn{name, left_name, right_name, body}) => {
        Expr::Literal(Type::Infix(env.clone(),name.to_string(),left_name.to_string(),
          body.clone(), right_name.to_string()))
      },
      Expr::DefnPrefix(PrefixDefn{name, params, body}) => {
        Expr::Literal(Type::Prefix(env.clone(),name.to_string(),params.to_vec(), body.clone()))
      },
    }
  }

  pub fn equals(&self, o: &Self) -> bool {
    unimplemented!()
  }

  pub fn new_env() -> Env {
    HashMap::new()
  }
}

#[test]
fn test_eval() {
  let test_int = 3;
  let expr = Expr::Assign("x".to_string(), Box::new(Expr::Literal(Type::Int(test_int))),
    Box::new(Expr::Variable("x".to_string())));
  if let Expr::Literal(Type::Int(a)) = expr.eval(&Expr::new_env()) {
    assert_eq!(test_int, a);
  } else {
    panic!("Failed at variable test")
  }
}
