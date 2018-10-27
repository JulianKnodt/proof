use ast::{Env, Type, Expr, GlobalEnv};
use std::sync::Arc;
use std::borrow::Borrow;
use std::collections::HashMap;

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
  pub fn default_global() -> GlobalEnv {
    let mut e = HashMap::new();
    e.insert(String::from("-"), Arc::new(Expr::Value(
      Type::new_rust_closure(|x: Vec<Arc<Type>>| {
        let mut items = x.iter();
        let first = items.next()
          .expect("Missing arguments, usage: (- [from: Number] [...values: Number])");
        if let Type::Number(n) = first.borrow() {
          Type::Number(items.fold(*n, |acc, v| match v.borrow() {
            Type::Number(num) => acc - num,
            _ => panic!("Cannot sub values which aren't numbers"),
          }))
        } else {
          panic!("First element must be of type number");
        }
      }
    ))));

    e.insert(String::from("*"), Arc::new(Expr::Value(
      Type::new_rust_closure(|x|
        Type::Number(x.iter().fold(1.0, |acc, elem| match elem.borrow() {
          Type::Number(n) => acc * n,
          _ => panic!("Cannot multiply by non-number"),
    }))))));

    e.insert(String::from("="), Arc::new(Expr::Value(
      Type::new_rust_closure(|x| {
        let mut items = x.iter();
        let first = items.next().expect("Missing arguments, usage: (= [comp] [... to])");
        Type::Bool(items.all(|i| i.equals(first)))
    }))));
    e
  }
}
