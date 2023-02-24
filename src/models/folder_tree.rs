
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

    let root_name: String = {
      let path_holder = path.trim_start_matches("/");
      let mut tmp: Vec<&str> = path_holder.split('/').collect();
      println!("{}", tmp[0]);
      let result = tmp[0];
      tmp.remove(0);
      result.to_string()
    };
    path.replace_range(..root_name.len(), "");

    let (nodes, leafs): (FolderTreeList, Vec<&'static str>)
      = Self::get_tree_structure(path);

    Self {
      name: root_name,
      folders: nodes,
      files: leafs,
    }
  }

  fn get_tree_structure(path: String) -> (FolderTreeList, Vec<&'static str>) {
    (vec![], vec![])
  }

  pub fn contains_file<'a>(&'a self, path: &[&'a str]) -> bool {
    let path_size: usize = path.len();


    let node: &FolderNode = self;
    let file: &str = path[path_size-1];
    let mut path: Vec<&'a str> = path.try_into().expect("Could not get a Vec from the array");
    path.pop();

    for s in path {
      
    }

    if node.name != file {
      false
    } else {
      true
    }

  }

  pub fn contains_folder<'a>(&'a self, path: &[&'a str]) -> bool {
    todo!();
  }

}
