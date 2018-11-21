pub mod compile;

#[cfg(test)]
mod tests {
  use compile::compile;
  use lisp_parse::Token;
  use std::process::Command;
  use std::fs::File;
  use std::path::Path;
  use std::str;

  fn basic_test_cases() -> Vec<(Token, &'static str)> {
    vec!(
      (Token::from("nil"), "nil"),
      (Token::from("3"), "3"),
      (Token::from("-3"), "-3"),
      (Token::from("0"), "0"),
      (Token::from("#\\a"), "#\\a"),
      (Token::from("#\\A"), "#\\A"),
      (Token::from("#t"), "#t"),
      (Token::from("#f"), "#f"),
    )
  }

  fn run_on(cases: Vec<(Token, &'static str)>) {
    let errors: Vec<String> = cases.into_iter().filter_map(|(input, expected)| {
      let mut file = File::create("temp_test.s").expect("Cannot open temp file");
      compile::compile(input, &mut file).expect("Could not compile");
      let comp_out = Command::new("gcc")
        .arg(format!("{}/runtime_test/runtime.c",
        Path::new(file!()).parent().unwrap().to_str().unwrap()))
        .arg("./temp_test.s")
        .output()
        .expect("Failed to compile test");
      if let Ok(err) = str::from_utf8(&comp_out.stderr) {
        if err != "" {
          return Some(String::from(err));
        }
      };

      let result = Command::new("./a.out").output().unwrap();
      if let Ok(err) = str::from_utf8(&result.stderr) {
        if err != "" {
          return Some(String::from(err))
        }
      };

      if let Ok(output) = str::from_utf8(&result.stdout) {
        if output.trim() == expected.trim() {
          None
        } else {
          Some(format!("Expected: {}, Got: {}", expected, output))
        }
      } else {
        panic!("Could not parse result");
      }
    }).collect();
    if !errors.is_empty() {
      println!("{:?}", errors);
      panic!("-------- \n{}\n", errors.join("\n"))
    }
  }

  #[test]
  fn run_tests() {
    run_on(basic_test_cases());
    // run_on(...)
  }
}
