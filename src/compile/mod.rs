pub mod compile;

#[cfg(test)]
mod tests {
  use compile::compile;
  use std::process::Command;
  use std::fs::File;
  use std::path::Path;
  use std::str;

  fn basic_test_cases() -> Vec<(&'static str, &'static str)> {
    vec!(
      ("nil", "nil"),
      ("()", "nil"),
      ("3", "3"),
      ("-3", "-3"),
      ("0", "0"),
      ("#\\a", "#\\a"),
      ("#\\A", "#\\A"),
      ("#t", "#t"),
      ("#f", "#f"),
    )
  }

  fn first<T>(a: Vec<T>) -> T {
    a.into_iter().next().unwrap()
  }

  fn one_arg_test_cases() -> Vec<(&'static str, &'static str)> {
    vec!(
      ("(fxadd1 1)", "2"),
      ("(fxadd1 (fxadd1 1))", "3"),
      ("(fxsub1 (fxsub1 3))", "1"),
      ("(char->fixnum #\\a)", "97"),
      ("(char->fixnum #\\A)", "65"),
      ("(fixnum->char 65)", "#\\A"),
      ("(fxzero? 0)", "#t"),
      ("(fxzero? 1)", "#f"),
      ("(null? 1)", "#f"),
      ("(null? ())", "#t"),
      ("(not ())", "#f"),
      ("(not #f)", "#t"),
      ("(fixnum? #f)", "#f"),
      ("(fixnum? 1)", "#t"),
      ("(fixnum? 0)", "#t"),
    )
  }

  fn run_on(cases: Vec<(&'static str, &'static str)>, name: &'static str) {
    use lisp_parse::parse;

    let errors: Vec<String> = cases.into_iter().enumerate().filter_map(|(i, (input, expected))| {
      let filename = format!("tmp{}_{}.s", name, i);
      let mut file = File::create(&filename).expect("Cannot open temp file");
      compile::compile(&first(parse(String::from(input))), &mut file)
        .expect("Could not compile");
      let newfile = format!("exe_{}_{}", name, i);
      let comp_out = Command::new("gcc")
        .arg(format!("{}/runtime_test/runtime.c",
        Path::new(file!()).parent().unwrap().to_str().unwrap()))
        .arg(filename)
        .arg("-o")
        .arg(&newfile)
        .output()
        .expect("Failed to compile test");
      if let Ok(err) = str::from_utf8(&comp_out.stderr) {
        if err != "" {
          return Some(String::from(err));
        }
      };

      let result = Command::new(format!("./{}",newfile)).output().expect("Could not run");
      if let Ok(err) = str::from_utf8(&result.stderr) {
        if err != "" {
          return Some(String::from(err))
        }
      };

      if let Ok(output) = str::from_utf8(&result.stdout) {
        if output.trim() == expected.trim() {
          None
        } else {
          Some(format!("Input({}th): {}, Expected: {}, Got: {}", i, input, expected, output))
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
    run_on(basic_test_cases(), "basic");
    run_on(one_arg_test_cases(), "one_arg");
    // run_on(...)
  }
}
