pub struct Counter(u64);

impl Counter {
  pub fn new() -> Counter {
    Counter(0)
  }
  pub fn take(&mut self) -> u64 {
    let result = self.0;
    self.0 += 1;
    result
  }
}

