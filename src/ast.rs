use std::sync::Arc;
use std::ops::Deref;
use std::borrow::Borrow;
use std::collections::HashMap;

// A closure function to implement primitives like +
type RustClosureFn = fn(Vec<Arc<Type>>) -> Type;

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

  RustClosure(Arc<RustClosureFn>),
}

impl Type {
  pub fn new_empty_list() -> Arc<Type> {
    Arc::new(Type::List(Arc::new(List::End)))
  }
  pub fn new_number(n: f32) -> Arc<Type> {
    Arc::new(Type::Number(n))
  }
  pub fn unit() -> Arc<Type> {
    Arc::new(Type::Unit)
  }
  pub fn new_rust_closure(r: RustClosureFn) -> Arc<Type> {
    Arc::new(Type::RustClosure(Arc::new(r)))
  }
  pub fn cons(a: &Arc<Type>, b: &Arc<List>) -> Arc<List> {
    Arc::new(List::Cons(Arc::clone(a), Arc::clone(b)))
  }
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
pub enum Assign {
  Local(String, Arc<Expr>, Arc<Expr>),
  Global(String, Arc<Expr>),
}

#[derive(Debug, Clone)]
pub enum Expr {
  Value(Arc<Type>),
  Variable(String),
  Defn(Arc<Defn>),
  Call(Arc<Expr>, Vec<Arc<Expr>>),
  Assign(Assign),
  If(Arc<Expr>, Arc<Expr>, Arc<Expr>),
}

#[derive(Debug, Clone)]
pub struct Env {
  name: String,
  bind: Arc<Expr>,
  old: Arc<Option<Env>>,
}

impl Env {
  pub fn with(old: Arc<Option<Env>>, name: String, bind: Arc<Expr>) -> Arc<Option<Env>> {
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

pub type GlobalEnv = HashMap<String, Arc<Expr>>;

impl Expr {
  fn to_type(&self) -> Arc<Type> {
    match self {
      Expr::Value(v) => Arc::clone(v),
      _ => panic!("Not a type"),
    }
  }
  pub fn eval(&self, env: Arc<Option<Env>>, g_env: &mut GlobalEnv) -> Arc<Expr> {
    match self {
      Expr::Value(v) => Arc::new(Expr::Value(Arc::clone(v))),
      Expr::Variable(name) => match Env::lookup(env, name.to_string()) {
        Some(expr) => expr,
        None => match g_env.get(name) {
          Some(expr) => Arc::clone(expr),
          None => panic!("Free variable {}", name),
        },
      }
      Expr::Assign(a) => match a {
        Assign::Local(name, val, body) => {
          let evald = val.eval(Arc::clone(&env), g_env);
          body.eval(Env::with(env, name.to_string(), evald), g_env)
        },
        Assign::Global(name, val) => {
          let evald = val.eval(Arc::clone(&env), g_env);
          g_env.insert(name.to_string(), evald);
          Arc::new(Expr::Value(Type::unit()))
        },
      },
      Expr::Defn(defn) => Arc::new(Expr::Value(Arc::new(Type::Closure(env, Arc::clone(defn))))),
      Expr::If(cond, pred, fallback) => match cond.eval(Arc::clone(&env), g_env).deref() {
        Expr::Value(inner) => match inner.borrow() {
          Type::Bool(true) => pred.eval(env, g_env),
          _ => fallback.eval(env, g_env),
        },
        _ => fallback.eval(env, g_env),
      },
      Expr::Call(operator, operands) => match operator.eval(Arc::clone(&env), g_env).deref() {
        Expr::Value(inner) => match inner.borrow() {
          Type::Closure(clos_env, defn) => {
            let fn_env = Env::with(Arc::clone(clos_env), defn.name.to_string(),
              Arc::clone(&defn.body));
            let fn_env = {
              let args = &mut operands.iter().map(|n| n.eval(Arc::clone(&env), g_env));
              defn.params.iter().fold(fn_env, move |e,p| match p {
                ParamType::Singular(name) =>
                  Env::with(e, name.to_string(), Arc::clone(&args.next()
                    .expect("Not enough args passed to function"))),

                ParamType::Rest(name) => Env::with(e, name.to_string(),
                  Arc::new(Expr::Value(Arc::new(Type::List(args.fold(Arc::new(List::End), |r,n|
                    Arc::new(List::Cons(n.to_type(),Arc::clone(&r))))))))),
              })
            };

            defn.body.eval(fn_env, g_env)
          },
          Type::RustClosure(func) => Arc::new(Expr::Value(Arc::new(func(operands.iter()
            .map(|it| it.eval(Arc::clone(&env), g_env).to_type()).collect())))),
          _ => panic!("Cannot invoke non-function")
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
  let out = expr.eval(Arc::new(None), HashMap::new());
  println!("{:?}", out);
}


