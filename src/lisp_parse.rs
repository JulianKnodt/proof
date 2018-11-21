#[derive(Debug)]
pub enum Token {
  Word(String),
  Group(Vec<Token>),
}

impl Token {
  pub fn from(s: &str) -> Token {
    Token::Word(String::from(s))
  }
  fn add_next(&mut self, next: Token) {
    match self {
      Token::Word(_) => panic!("Cannot add next to singleton"),
      Token::Group(ref mut tg) => tg.push(next),
    }
  }
  fn init_group() -> Self {
    Token::Group(Vec::new())
  }
}

pub fn parse(body: String) -> Vec<Token> {
  let to_parse = body.trim();
  let mut done: Vec<Token> = Vec::new();
  let mut buf: Vec<Token> = Vec::new();
  let mut curr = String::from("");
  for c in to_parse.chars() {
    match c {
      '(' => buf.push(Token::init_group()),
      ')' => {
        if curr.len() > 0 {
          let len = buf.len() - 1;
          buf[len].add_next(Token::Word(curr.clone()));
          curr = String::from("");
        }
        let completed = buf.pop().expect("Extra right parens");
        if buf.is_empty() {
          done.push(completed)
        } else {
          let len = buf.len() - 1;
          buf.get_mut(len).expect("Extra right parens").add_next(completed);
        }
      },
      s if s.is_whitespace() && curr.len() == 0 => (),
      s if s.is_whitespace() => {
        if buf.is_empty() {
          done.push(Token::Word(curr.clone()));
        } else {
          let len = buf.len() - 1;
          buf[len].add_next(Token::Word(curr.clone()));
        }
        curr = String::from("");
      },
      x => curr.push(x),
    }
  };
  if curr.len() > 0 {
    // Should only happen in the case where the line is just a single token
    done.push(Token::Word(curr.clone()));
  }
  if buf.len() > 0 {
    panic!("Unmatched left parens");
  }
  match buf.pop() {
    None => (),
    Some(group) => done.push(group),
  }
  done
}

#[test]
fn test_parse() {
  let tokens = parse(String::from("(+ (+ 2 yes) 1)"));
  println!("{:?}", tokens);
}
