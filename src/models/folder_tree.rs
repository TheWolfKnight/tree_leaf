
use std::fs::{File, self};

pub type FolderTreeList = Vec<Box<FolderNode>>;

#[derive(Clone, Debug)]
pub struct FolderNode {
  pub name: String,
  pub folders: FolderTreeList,
  pub files: Vec<&'static str>,
}

impl FolderNode {
  pub fn new<'a>(path: impl Into<String>) -> Self {
    let mut path: String = path.into();
    path = path.trim_start_matches("/").replace("\\", "/").to_string();

    let result = Self {
      name: path.clone(),
      folders: vec![],
      files: vec![],
    };

    Self::get_tree_structure(path, &result);

    result
  }

  fn get_tree_structure(path: String, first_node: &FolderNode) {

    let mut holder: Vec<(usize, &FolderNode)> = Vec::new();
    holder.push((0, first_node));

    while !holder.is_empty() {
    }
  }

  fn new_for_tree_gen(name: String) -> Self {
    Self {
      name,
      folders: Vec::new(),
      files: Vec::new(),
    }
  }

  pub fn contains_file<'a>(&'a self, path: &[&'a str]) -> bool {
    let path_size: usize = path.len();

    let mut node: &FolderNode = self;
    let file: &str = path[path_size-1];
    let mut path: Vec<&'a str> = path.try_into().expect("Could not get a Vec from the array");
    path.pop();

    for s in path {
        match node.folders.iter().find(|f| f.name == s) {
          Some(n) => node = n,
          None => return false,
        }
      }

    match node.folders.iter().find(|f| f.name == file) {
      Some(_) => return true,
      None => return false
    }
  }

  pub fn contains_folder<'a>(&'a self, path: &[&'a str]) -> bool {
    let mut node: &FolderNode = self;

    for s in path {
        match node.folders.iter().find(|f| f.name == *s) {
          Some(n) => node = n,
          None =>  return false,
        }
    }
    true
  }
}
