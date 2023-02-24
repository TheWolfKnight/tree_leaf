
pub struct WebPath {
  pub prefix: String,
}

impl WebPath {
  pub fn new<T: Into<String>>(prefix: T) -> Self {
    let prefix: String = prefix.into();
    Self { prefix }
  }

  pub fn parse_path<'a, T: Into<&'a str>>(&self, path: T) -> Option<Vec<&'a str>> {
    let mut path: &'a str = path.into();
    let trimed_path: Option<&str> = path.strip_prefix(self.prefix.as_str());

    match trimed_path {
      Some(p) => path = p,
      None => { return None; }
    }

    Some(path.split('/')
             .collect::<Vec<&str>>())
  }
}

