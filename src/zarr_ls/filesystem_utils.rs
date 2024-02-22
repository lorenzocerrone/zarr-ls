use super::common::{ActionSelect, BACK};
use super::zarr_utils::{ZarrIdentifier, ZarrNode};
use std::path::PathBuf;

use indexmap::IndexMap;

use std::fs;

static ZARR_EXTENSION: &str = "zarr";

fn read_dir(path: PathBuf) -> Vec<PathBuf> {
    assert!(path.exists());
    let paths = fs::read_dir(path);
    paths
        .unwrap()
        .map(|entry| entry.unwrap().path())
        // Check if entry is a hidden file or a directory
        .filter(|entry| {
            !entry
                .file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with('.')
        })
        // Check if entry is a directory or a zarr file
        .filter(|entry| entry.is_dir() || entry.ends_with(".zarr"))
        .collect::<Vec<PathBuf>>()
}

fn path_to_selection(path: PathBuf) -> ActionSelect {
    let path_extension = match path.extension() {
        Some(ext) => ext.to_str().unwrap(),
        None => "",
    };

    if path_extension == ZARR_EXTENSION {
        let zarr_identifier = ZarrIdentifier::ZarrPath(path.clone());
        let zarr_node = ZarrNode::new(&zarr_identifier);
        return ActionSelect::Zarr(zarr_node);
    }

    if path.is_dir() {
        let dir_node = DirNode::new(&path);
        return ActionSelect::Dir(dir_node);
    }

    ActionSelect::Error("This should not happen".to_string())
}

#[derive(Debug, Clone)]
pub struct DirNode {
    path: PathBuf,
}

impl DirNode {
    pub fn new(path: &PathBuf) -> Self {
        DirNode { path: path.clone() }
    }

    pub fn get_options(&self) -> IndexMap<String, ActionSelect> {
        let paths: Vec<PathBuf> = read_dir(self.path.clone());
        let mut matching_name: IndexMap<String, ActionSelect> = IndexMap::new();

        for path in paths {
            let path_name = path.file_name().unwrap().to_str().unwrap().to_string();
            let current_selection = path_to_selection(path.clone());
            matching_name.insert(path_name, current_selection);
        }
        matching_name
    }

    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn parent(&self) -> Option<PathBuf> {
        self.path.parent().map(|path| path.to_path_buf())
    }
}
