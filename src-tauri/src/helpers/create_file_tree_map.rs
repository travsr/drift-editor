use std::{collections::HashMap, fs, path::Path};

use crate::models::d_file_tree_node::DFileTreeNode;

pub fn create_file_tree_map(root_path: &str) -> HashMap<String, DFileTreeNode> {
    let mut map = HashMap::new();

    fn walk(path: &Path, map: &mut HashMap<String, DFileTreeNode>) -> Option<String> {
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
                    if let Some(child_id) = walk(&child_path, map) {
                        children.push(child_id);
                    }
                }
            }
        }

        map.insert(
            id.clone(),
            DFileTreeNode {
                id: id.clone(),
                name,
                is_expanded: Some(true),
                children: children,
            },
        );

        Some(id)
    }

    walk(Path::new(root_path), &mut map);
    map
}
