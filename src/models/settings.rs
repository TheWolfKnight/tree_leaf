
pub struct Settings<'a> {
  pub target: &'a str,
  pub prefix: &'a str,
  pub verbose: bool,
  pub worker_pool_size: usize,
}

impl<'a> Settings<'a> {

  pub fn new() -> Self {
    Self {
      target: "127.0.0.1:8080",
      prefix: "./",
      verbose: false,
      worker_pool_size: 4,
    }
  }

  pub fn set_target(&mut self, target: &'static str) -> () {
    self.target = target;
  }

  pub fn set_prefix(&mut self, prefix: &'static str) -> () {
    self.prefix = prefix;
  }

  pub fn set_verbose(&mut self, is_verbose: bool) -> () {
    self.verbose = is_verbose;
  }
}
