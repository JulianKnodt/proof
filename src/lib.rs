#![feature(slice_patterns)]

#[macro_use]
extern crate lazy_static;

pub mod ast;
pub mod lisp_parse;
pub mod to_ast;
pub mod default_env;
pub mod equals;
pub mod compile;
