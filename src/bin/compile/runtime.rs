extern crate csv;

use std::error::Error;
use std::fs::File;

extern "C" {
  fn scheme() -> i64;
}

pub fn main() {
  match run_tests() {
    Ok(()) => println!("Ok."),
    Err(error) => println!("{}", error),
  }
  // let result = unsafe { scheme() };
}

pub fn run_tests() -> Result<(), Box<Error>> {
  let file = File::open("tests.csv")?;
  let mut rdr = csv::Reader::from_reader(file);
  for result in rdr.records() {
    let record = result?;
    println!("{:?}", record);
  }
  Ok(())
}
