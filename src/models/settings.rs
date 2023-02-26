
use std::{
  env::{current_dir, self},
  process::exit,
  path::PathBuf,
};

pub struct Settings<'a> {
  pub target: &'a str,
  pub prefix: &'a str,
  pub verbose: bool,
  pub worker_pool_size: usize,
}

impl<'a> Settings<'a> {

  pub fn new() -> Self {
    let current_working_dir: Result<PathBuf, std::io::Error> = current_dir();

    if let Err(e) = current_working_dir {
      eprintln!("[ERROR] Could not get current working dirrectory: {}", e);
      exit(1);
    }

    let dir_name = current_working_dir.unwrap().to_str().unwrap();

    Self {
      target: "127.0.0.1:8080",
      prefix: dir_name,
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
