// This file defines custom types and structures used throughout the application, 
// such as data types for files and folders.

pub struct File {
    pub name: String,
    pub path: String,
    pub is_directory: bool,
}

pub struct Folder {
    pub name: String,
    pub path: String,
    pub files: Vec<File>,
    pub is_expanded: bool,
}

#[derive(Debug)]
pub struct FileNode {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub children: Vec<FileNode>,
    pub is_expanded: bool,
}

#[derive(Debug, Clone)]
pub struct FlatItem {
    pub idx_path: Vec<usize>,   // path of indices from root to this node
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub is_expanded: bool,
    pub indent: u16,            // how deep to indent when rendering
}