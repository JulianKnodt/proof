use ast::{Env, Type, Expr, GlobalEnv, List};
use std::sync::Arc;
use std::borrow::Borrow;
use std::collections::HashMap;

impl Env {
  pub fn default() -> Arc<Option<Env>> {
    let e = Arc::new(None);
    let e = Env::with(e, String::from("+"), Arc::new(Expr::Value(
      Type::new_rust_closure(|x: Vec<Arc<Type>>|
        Type::new_number(x.iter().fold(0.0, |acc, elem| match elem.borrow() {
      Type::Number(n) => acc + n,
      _ => panic!("Cannot add non-number"),
    }))))));

    let e = Env::with(e, String::from("cons"), Arc::new(Expr::Value(
      Type::new_rust_closure(|x: Vec<Arc<Type>>|{
      let mut items = x.iter().rev();
      let first = items.next().expect("Missing arguments, usage: cons [...items] [into list]");
      if let Type::List(sub) = first.borrow() {
        Arc::new(Type::List(items.fold(Arc::clone(sub), |l, next| Type::cons(next, &l))))
      } else {
        panic!("Last element was expected to be array");
      }
    }))));

    let e = Env::with(e, String::from("debug"), Arc::new(Expr::Value(
      Type::new_rust_closure(|x: Vec<Arc<Type>>| {
      x.iter().for_each(|item| println!("?:{:?}", item));
      Type::unit()
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
          Type::new_number(items.fold(*n, |acc, v| match v.borrow() {
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
        Type::new_number(x.iter().fold(1.0, |acc, elem| match elem.borrow() {
          Type::Number(n) => acc * n,
          _ => panic!("Cannot multiply by non-number"),
    }))))));

    e.insert(String::from("="), Arc::new(Expr::Value(
      Type::new_rust_closure(|x| {
        let mut items = x.iter();
        let first = items.next().expect("Missing arguments, usage: (= [comp] [... to])");
        Arc::new(Type::Bool(items.all(|i| i.equals(first))))
    }))));

    e.insert(String::from("hd"), Arc::new(Expr::Value(
      Type::new_rust_closure(|x| match x.get(0) {
        None => panic!("Missing arguments, usage: (hd [from: List])"),
        Some(v) => if let Type::List(l) = v.borrow() {
          match l.borrow() {
            List::End => Arc::clone(v),
            List::Cons(a, _) => Arc::clone(a),
          }
        } else {
          panic!("Argument incorrect type, expected list, got {:?}", v)
        }
      }
    ))));

    e.insert(String::from("tl"), Arc::new(Expr::Value(
      Type::new_rust_closure(|x| match x.get(0) {
        None => panic!("Missing arguments, usage: (tl [from: List])"),
        Some(v) => if let Type::List(l) = v.borrow() {
          match l.borrow() {
            List::End => Arc::clone(v),
            List::Cons(_, b) => Arc::new(Type::List(Arc::clone(b))),
          }
        } else {
          panic!("Argument incorrect type, expected list, got {:?}", v)
        }
      }
    ))));

    e
  }
}
