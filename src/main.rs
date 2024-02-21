use inquire::Select;
use std::collections::HashMap;
use std::fmt::Error;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use zarrs::metadata;
use zarrs::node::Node;
use zarrs::node::NodePath;
use zarrs::storage::store::{self, FilesystemStore};
use zarrs::storage::{get_child_nodes, meta_key};

#[derive(Debug, Clone)]
enum CurrentSelection {
    Directory(PathBuf),
    ZarrFile(Node),
    Exit,
}

fn find_files(path: PathBuf) -> Vec<PathBuf> {
    assert!(path.exists());
    assert!(path.is_dir());

    let paths = read_dir(path);
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

fn find_files_options(path: PathBuf) -> HashMap<String, CurrentSelection> {
    let paths: Vec<PathBuf> = find_files(path.clone());
    let mut matching_name: HashMap<String, CurrentSelection> = HashMap::new();

    for path in paths {
        let path_name = path.file_name().unwrap().to_str().unwrap().to_string();
        let current_selection = if path.ends_with(".zarr") {
            CurrentSelection::Directory(path)
        } else {
            let store = Arc::new(FilesystemStore::new(path).unwrap());
            let node = Node::new(&*store, "/").unwrap();
            CurrentSelection::ZarrFile(node)
        };
        matching_name.insert(path_name, current_selection);
    }

    // Special options
    let parent_path = path.parent().unwrap().to_path_buf();
    matching_name.insert(
        "..".to_string(),
        CurrentSelection::Directory(parent_path.clone()),
    );
    matching_name.insert("Exit!".to_string(), CurrentSelection::Exit);
    return matching_name;
}

fn find_zarr_options(node: Node) -> HashMap<String, CurrentSelection> {
    let mut matching_name: HashMap<String, CurrentSelection> = HashMap::new();
    for child in node.children() {
        let child_name = child.name().to_string();
        let current_selection = CurrentSelection::ZarrFile(child.clone());
        matching_name.insert(child_name, current_selection);
    }
    matching_name.insert("Exit!".to_string(), CurrentSelection::Exit);
    return matching_name;
}

fn build_menu(current_path: CurrentSelection) -> CurrentSelection {
    let matching_selections: HashMap<String, CurrentSelection> = match current_path {
        CurrentSelection::Directory(dir_path) => find_files_options(dir_path),
        CurrentSelection::ZarrFile(zarr_path) => find_zarr_options(zarr_path),
        CurrentSelection::Exit => panic!("Exit should not be a valid option"),
    };

    let options: Vec<String> = matching_selections.keys().cloned().collect();
    let selection = Select::new("Select a file or zarr group", options).prompt();
    let selected_option_name = selection.unwrap();
    matching_selections
        .get(&selected_option_name)
        .unwrap()
        .clone()
}

fn main() {
    let mut path: PathBuf = PathBuf::from("/home/lcerrone/").canonicalize().unwrap();
    let mut current_selection: CurrentSelection = CurrentSelection::Directory(path);
    loop {
        match current_selection {
            CurrentSelection::Exit => break,
            _ => {
                current_selection = build_menu(current_selection);
            }
        }
    }
}
