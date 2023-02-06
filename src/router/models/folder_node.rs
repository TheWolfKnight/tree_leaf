
use super::file_node::FileNode;

pub type FileVec = Box<Vec<FileNode>>;
pub type FolderVec = Box<Vec<FolderNode>>;

pub struct FolderNode{
  pub files: FileVec,
  pub folders: FolderVec,
}

impl FolderNode {
}
