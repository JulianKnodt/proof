use ast::{Type, List};

impl Type {
  pub fn equals(&self, o: &Self) -> bool {
    match self {
      Type::Unit => if let Type::Unit = o { true } else { false },
      Type::Number(a) => if let Type::Number(b) = o { a == b } else { false },
      Type::Bool(a) => if let Type::Bool(b) = o { a == b } else { false },
      Type::Str(a) => if let Type::Str(b) = o { a == b } else { false },
      Type::Tuple(a, b) =>
        if let Type::Tuple(c, d) = o { a.equals(c) && b.equals(d) } else { false },
      Type::List(a) => if let Type::List(b) = o { a.equals(b) } else { false },
      Type::Free(_) => unimplemented!(),
        //if let Type::Free(b) = o { a.equals(b) } else { false },
      _ => false,
    }
  }
}

impl List {
  pub fn equals(&self, o: &Self) -> bool {
    match self {
      List::End => if let List::End = o { true } else { false },
      List::Cons(hd, tl) => if let List::Cons(h2, t2) = o {
        hd.equals(h2) && tl.equals(t2)
      } else { false },
    }
  }
}
