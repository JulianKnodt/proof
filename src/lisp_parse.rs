#[derive(Debug)]
pub enum Token {
  Word(String),
  Group(Vec<Token>),
}

impl Token {
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

pub fn parse(body: String) -> Token {
  let to_parse = body.trim();
  let mut buf: Vec<Token> = vec!(Token::init_group());
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
        let len = buf.len() - 1;
        buf.get_mut(len).expect("Extra right parens").add_next(completed);
      },
      s if s.is_whitespace() && curr.len() == 0 => (),
      s if s.is_whitespace() => {
        let len = buf.len() - 1;
        buf[len].add_next(Token::Word(curr.clone()));
        curr = String::from("");
      },
      x => curr.push(x),
    }
  };
  if buf.len() > 1 {
    panic!("Unmatched left parens");
  }
  match buf.pop().unwrap() {
    Token::Word(_) => panic!("Expected first elem to be group"),
    Token::Group(mut tg) => tg.pop().unwrap(),
  }
}

#[test]
fn test_parse() {
  let tokens = parse(String::from("(+ (+ 2 yes) 1)"));
  println!("{:?}", tokens);
}
