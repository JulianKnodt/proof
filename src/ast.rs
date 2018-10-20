use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum ParamType {
  Singular(String),
  Rest(String),
}

#[derive(Clone)]
pub enum Type {
  Free(Box<Expr>),
  // Special cases
  Unit,

  Int(i32),
  Float(f32),
  Bool(bool),
  Str(String),
  Tuple(Arc<Type>, Arc<Type>),

  Closure(Env, Defn),

  List(Arc<List>),

  // This is for implementing primitives like add
  RustClosure(&'static Fn(Vec<Type>) -> Type),
}

impl fmt::Debug for Type {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Type::Int(i) => write!(f, "{}", i),
      Type::Float(i) => write!(f, "{}", i),
      Type::Bool(v) => write!(f, "{}", v),
      Type::Unit => write!(f, "()"),
      Type::Free(expr) => write!(f, "{:?}", expr),
      Type::Str(string) => write!(f, "{}", string),
      Type::RustClosure(_) => write!(f, "Anonymous Closure"),
      _ => write!(f, "TODO Lol"),
    }
  }
}


#[derive(Clone, Debug)]
pub enum List {
  End,
  Cons(Type, Arc<List>),
}


#[derive(Clone, Debug)]
pub struct Defn {
  pub name: String,
  pub params: Vec<ParamType>, // the name which the list of arguments will be bound to
  pub body: Box<Expr>
}

//impl fmt::Debug for Defn {
//  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//    write!(f, "Fn({})", self.name)
//  }
//}

#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub enum Expr {
  Literal(Type),
  Variable(String),
  Defn(Defn), // define 1st as prefix of 2nd in Exp
  Call(Box<Expr>, Vec<Expr>), // call 1st on rest of Expressions
  Assign(String, Box<Expr>, Box<Expr>), // assign to name value of 1st expression in 2nd
  Match(Box<Expr>, Vec<(MatchPatterns, Expr)>), // compare value of 1st expr to each of 2nd
}

// TODO turn this into an Arc instead of an Expr
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
      Expr::Call(func, args) => {
        let prefix = func.eval(env);
        match prefix.clone() {
          Expr::Literal(Type::Closure(clos,Defn{name, params, body})) => {
            let mut arg_iter = args.iter();
            params.iter().fold(&env, |acc, param| match param {
              ParamType::Singular(name) => {
                let next_arg = arg_iter.next().expect("Not enough arguments supplied to {}",
                  name);
                env_with(acc, name, next_arg.eval(env))
              },
              ParamType::Rest(rest_name) => {
                env_with(acc, rest_name, arg_iter.fold(List::End, |acc, n| List::Cons(n, acc)))
              },
            }
          },
          non_func => panic!("Cannot apply non-prefix func {:?}", non_func),
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
      Expr::Defn(defn) => Expr::Literal(Type::Closure(env.clone(), defn)),
    }
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

