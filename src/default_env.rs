use ast::{Defn, Type, Expr, ParamType};
use std::iter;
use std::sync::Arc;

impl Defn {
  pub fn add_def() -> Self {
    Defn{
      name: String::from("+"),
      params: vec!(ParamType::Rest(String::from("parts"))),
      body: unimplemented!(),
    }
  }
}
