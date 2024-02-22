use indexmap::IndexMap;
use inquire::Select;
use zarrs::array::ArrayMetadata;

use zarrs::group::GroupMetadata;

use std::collections::LinkedList;
use std::fs;
use std::path::PathBuf;

use zarrs::node::{Node, NodeMetadata};
use std::env;
use clap::Parser;

use zarrs::storage::store::FilesystemStore;

static ZARR_GROUP_HEADER: &str = "Zarr Group: ";
static ZARR_ARRAY_HEADER: &str = "Zarr Array: ";
static ZARR_EXTENSION: &str = "zarr";
static EXIT: &str = "Exit!";
static BACK: &str = "..";

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

fn path_to_selection(path: PathBuf) -> CurrentSelection {
    let path_extension = match path.extension() {
        Some(ext) => ext.to_str().unwrap(),
        None => "",
    };

    if path_extension == ZARR_EXTENSION {
        let node = zarr_node_from_path(path.clone());
        return CurrentSelection::ZarrFile(node);

    }

    if path.is_dir() {
        return CurrentSelection::Directory(path);
    }

    panic!("Invalid path: {:?}. It should be either a directory or a zarr file", path);
    
}

fn find_files_options(path: PathBuf) -> IndexMap<String, CurrentSelection> {
    let paths: Vec<PathBuf> = read_dir(path.clone());
    let mut matching_name: IndexMap<String, CurrentSelection> = IndexMap::new();

    for path in paths {
        let path_name = path.file_name().unwrap().to_str().unwrap().to_string();
        let current_selection = path_to_selection(path.clone());
        matching_name.insert(path_name, current_selection);
    }
    
    let parent_path = path.parent().map(|path| path.to_path_buf());
    
    matching_name.insert(BACK.to_string(), CurrentSelection::Back(parent_path));
    matching_name.insert(EXIT.to_string(), CurrentSelection::Exit);
    matching_name
}


struct FileWalker {
    current_path: PathBuf,
}


impl FileWalker {
    pub fn new(path: &PathBuf) -> Self {
        FileWalker { current_path: path.clone() }
    }

    pub fn get_options(&self) -> IndexMap<String, PathBuf> {
        let paths: Vec<PathBuf> = read_dir(path.clone());
        let mut matching_name: IndexMap<String, CurrentSelection> = IndexMap::new();
    
        for path in paths {
            let path_name = path.file_name().unwrap().to_str().unwrap().to_string();
            let current_selection = path_to_selection(path.clone());
            matching_name.insert(path_name, current_selection);
        }
        
        let parent_path = path.parent().map(|path| path.to_path_buf());
        matching_name
    }

    pub fn previous(&mut self) {
        let previous_node = self.visited_nodes.pop_back().unwrap();
        self.current_node = previous_node;
    }

    pub fn set_next(&mut self, node: Node) {
        self.visited_nodes.push_back(self.current_node);
        self.current_node = node;
    }
}