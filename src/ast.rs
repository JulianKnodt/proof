use std::sync::Arc;
use std::fmt;
use std::ops::Deref;
use std::borrow::Borrow;

type RustClosureFn = &'static Fn(Vec<Type>) -> Type;
impl fmt::Debug for RustClosureFn {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Rust Closure")
  }
}

#[derive(Debug, Clone)]
pub enum Type {
  Unit,
  Free(Arc<Expr>),
  Number(f32),
  Str(String),
  Bool(bool),
  Tuple(Arc<Type>, Arc<Type>),

  Closure(Arc<Option<Env>>, Arc<Defn>),

  List(Arc<List>),

  RustClosure(RustClosureFn),
}

#[derive(Debug, Clone)]
pub enum List {
  End,
  Cons(Arc<Type>, Arc<List>),
}

#[derive(Debug, Clone)]
pub enum ParamType {
  Singular(String),
  Rest(String),
}

#[derive(Debug, Clone)]
pub struct Defn {
  pub name: String,
  pub params: Vec<ParamType>,
  pub body: Arc<Expr>,
}

#[derive(Debug, Clone)]
pub enum Expr {
  Value(Arc<Type>),
  Variable(String),
  Defn(Arc<Defn>),
  Call(Arc<Expr>, Vec<Arc<Expr>>),
  Assign(String, Arc<Expr>, Arc<Expr>),
  If(Arc<Expr>, Arc<Expr>, Arc<Expr>),
}

#[derive(Debug, Clone)]
pub struct Env {
  name: String,
  bind: Arc<Expr>,
  old: Arc<Option<Env>>,
}

impl Env {
  fn with(old: Arc<Option<Env>>, name: String, bind: Arc<Expr>) -> Arc<Option<Env>> {
    Arc::new(Some(Env{name, bind, old}))
  }
  fn lookup(env: Arc<Option<Env>>, name: String) -> Option<Arc<Expr>> {
    let e = if let Some(v) = env.borrow() { v } else { return None };
    if e.name == name {
      return Some(Arc::clone(&e.bind))
    }
    Env::lookup(Arc::clone(&e.old), name)
  }
}

impl Expr {
  fn to_type(&self) -> Arc<Type> {
    match self {
      Expr::Value(v) => Arc::clone(v),
      _ => panic!("Not a type"),
    }
  }
  pub fn eval(&self, env: Arc<Option<Env>>) -> Arc<Expr> {
    match self {
      Expr::Value(v) => Arc::new(Expr::Value(Arc::clone(v))),
      Expr::Variable(name) => match Env::lookup(env, name.to_string()) {
        None => panic!("Free variable {}", name),
        Some(expr) => expr,
      }
      Expr::Assign(name, val, body) => {
        let evald = val.eval(Arc::clone(&env));
        body.eval(Env::with(env, name.to_string(), evald))
      },
      Expr::Defn(defn) => Arc::new(Expr::Value(Arc::new(Type::Closure(env, Arc::clone(defn))))),
      Expr::If(cond, pred, fallback) => match cond.eval(Arc::clone(&env)).deref() {
        Expr::Value(inner) => match inner.borrow() {
          Type::Bool(true) => pred.eval(env),
          _ => fallback.eval(env),
        },
        _ => fallback.eval(env),
      },
      Expr::Call(operator, operands) => match operator.eval(Arc::clone(&env)).deref() {
        Expr::Value(inner) => if let Type::Closure(clos_env, defn) = inner.borrow() {
          let fn_env = Env::with(Arc::clone(clos_env), defn.name.to_string(),
            Arc::clone(&defn.body));
          let args = &mut operands.iter().map(|n| n.eval(Arc::clone(&env)));

          let fn_env = defn.params.iter().fold(fn_env, move |e,p| match p {
            ParamType::Singular(name) =>
              Env::with(e, name.to_string(), Arc::clone(&args.next()
                .expect("Not enough args passed to function"))),

            ParamType::Rest(name) => Env::with(e, name.to_string(),
              Arc::new(Expr::Value(Arc::new(Type::List(args.fold(Arc::new(List::End), |r,n|
                Arc::new(List::Cons(n.to_type(),Arc::clone(&r))))))))),
          });

          defn.body.eval(fn_env)
        } else {
          panic!("Cannot invoke non-function")
        },
        _ => panic!("Cannot invoke non-function"),
      },
    }
  }
}

#[test]
fn test_basic() {
  let test_num = 3.0;
  let expr = Expr::Assign(String::from("x"),
    Arc::new(Expr::Value(Arc::new(Type::Number(test_num)))),
    Arc::new(Expr::Variable(String::from("x"))));
  let out = expr.eval(Arc::new(None));
  println!("{:?}", out);
}

