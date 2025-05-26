use std::{fs, path::Path};

use crate::models::d_file_tree_node::DFileTreeNode;

pub fn create_file_tree_vec(root_path: &str) -> Vec<DFileTreeNode> {
    let mut vector: Vec<DFileTreeNode> = vec![];

    fn walk(path: &Path, vector: &mut Vec<DFileTreeNode>, level: u16) -> Option<String> {
        let path_str = path.to_string_lossy(); // only allocated once when needed
        let id = path_str.to_string(); // used for ID and map key
        let name = path
            .file_name()
            .map(|s| s.to_string_lossy().into_owned())
            .unwrap_or_else(|| id.clone());

        let mut children = Vec::new();

        if path.is_dir() {
            if let Ok(entries) = fs::read_dir(path) {
                for entry in entries.flatten() {
                    let child_path = entry.path();
                    if let Some(child_id) = walk(&child_path, vector, level + 1) {
                        children.push(child_id);
                    }
                }
            }
        }

        vector.push(DFileTreeNode {
            id: id.clone(),
            name,
            is_expanded: Some(true),
            children: children,
            level,
        });

        Some(id)
    }

    walk(Path::new(root_path), &mut vector, 0);
    vector.reverse();
    vector
}
