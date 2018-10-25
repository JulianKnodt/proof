use ast::{Env, Type, Expr};
use std::sync::Arc;
use std::borrow::Borrow;

impl Env {
  pub fn default() -> Arc<Option<Env>> {
    let e = Arc::new(None);
    let e = Env::with(e, String::from("+"), Arc::new(Expr::Value(
      Type::new_rust_closure(|x: Vec<Arc<Type>>|
        Type::Number(x.iter().fold(0.0, |acc, elem| match elem.borrow() {
      Type::Number(n) => acc + n,
      _ => panic!("Cannot add non-number"),
    }))))));

    let e = Env::with(e, String::from("cons"), Arc::new(Expr::Value(
      Type::new_rust_closure(|x: Vec<Arc<Type>>|{
      let mut items = x.iter().rev();
      let first = items.next().expect("Missing arguments, usage: cons [...items] [into list]");
      if let Type::List(sub) = first.borrow() {
        Type::List(items.fold(Arc::clone(sub), |l, next| Type::cons(next, &l)))
      } else {
        panic!("Last element was expected to be array");
      }
    }))));

    let e = Env::with(e, String::from("debug"), Arc::new(Expr::Value(
      Type::new_rust_closure(|x: Vec<Arc<Type>>| {
      x.iter().for_each(|item| println!(":?{:?}", item));
      Type::Unit
    }))));
    e
  }
}
