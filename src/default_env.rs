use ast::{Env, Defn, Type, Expr};
use std::sync::Arc;
use std::borrow::Borrow;

impl Env {
  pub fn default() -> Arc<Option<Env>> {
    let e = Arc::new(None);
    let e = Env::with(e, String::from("+"), Arc::new(Expr::Value(
      Arc::new(Type::RustClosure(Arc::new(|x: Vec<Arc<Type>> |
        Type::Number(x.iter().fold(0.0, |acc, elem| match elem.borrow() {
      Type::Number(n) => acc + n,
      _ => panic!("Cannot add non-number"),
    }))))))));
    e
  }
}
