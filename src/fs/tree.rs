// This file defines the structure of the folder tree. It represents folders and files, including methods to expand or collapse folders.

use std::fs;
use std::path::{Path, PathBuf};

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
    pub idx_path: Vec<usize>, // path of indices from root to this node
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub is_expanded: bool,
    pub indent: u16, // how deep to indent when rendering
}

impl FileNode {
    pub fn new(name: String, path: PathBuf, is_dir: bool) -> Self {
        Self {
            name,
            path,
            is_dir,
            children: Vec::new(),
            is_expanded: false,
        }
    }

    pub fn expand(&mut self) {
        if self.is_dir && !self.is_expanded {
            self.children = Self::read_directory(&self.path);
            self.is_expanded = true;
        }
    }

    pub fn collapse(&mut self) {
        if self.is_expanded {
            self.children.clear();
            self.is_expanded = false;
        }
    }

    pub fn read_directory(path: &Path) -> Vec<FileNode> {
        let mut nodes = Vec::new();
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                let name = path.file_name().unwrap().to_string_lossy().into_owned();
                let is_dir = path.is_dir();
                nodes.push(FileNode::new(name, path, is_dir));
            }
        }
        nodes
    }
}

pub fn flatten(nodes: &[FileNode]) -> Vec<FlatItem> {
    fn walk(out: &mut Vec<FlatItem>, nodes: &[FileNode], prefix: &[usize], indent: u16) {
        for (i, node) in nodes.iter().enumerate() {
            let mut idx_path = prefix.to_vec();
            idx_path.push(i);
            // clone idx_path for storing in out while keeping a copy for recursion
            let idx_for_item = idx_path.clone();
            out.push(FlatItem {
                idx_path: idx_for_item,
                name: node.name.clone(),
                path: node.path.clone(),
                is_dir: node.is_dir,
                is_expanded: node.is_expanded,
                indent,
            });
            if node.is_dir && node.is_expanded {
                walk(out, &node.children, idx_path.as_slice(), indent + 1);
            }
        }
    }

    let mut out = Vec::new();
    walk(&mut out, nodes, &[], 0);
    out
}
