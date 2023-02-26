
use std::{
  env::{current_dir, self},
  process::exit,
  path::PathBuf,
};

pub struct Settings {
  pub target: String,
  pub prefix: String,
  pub verbose: bool,
  pub worker_pool_size: usize,
}

impl Settings {

  pub fn new() -> Self {
    let current_working_dir: Result<PathBuf, std::io::Error> = current_dir();

    if let Err(e) = current_working_dir {
      eprintln!("[ERROR] Could not get current working dirrectory: {}", e);
      exit(1);
    } else {
      let dir_name: String = current_working_dir.unwrap().to_str().unwrap().to_string();

      Self {
        target: "127.0.0.1:8080".to_string(),
        prefix: dir_name,
        verbose: false,
        worker_pool_size: 4,
      }
    }
  }

  pub fn set_target(&mut self, target: String) -> () {
    self.target = target;
  }

  pub fn set_prefix(&mut self, prefix: String) -> () {
    self.prefix = prefix;
  }

  pub fn set_verbose(&mut self, is_verbose: bool) -> () {
    self.verbose = is_verbose;
  }
}
