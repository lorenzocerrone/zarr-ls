use inquire::Select;
use std::collections::HashMap;
use std::fmt::Error;
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use zarrs::node::NodePath;
use zarrs::storage::store::{self, FilesystemStore};
use zarrs::storage::{get_child_nodes, meta_key};

enum PathResult {
    ZarrFile(PathBuf),
    Directory(PathBuf),
    Exit,
    Error(Error),
}

fn WalkZarr(path: PathBuf) {
    let store = Arc::new(FilesystemStore::new(path).unwrap());
    // Show the hierarchy
    // let node = Node::new(&*store, "/").unwrap();
    // let tree = node.hierarchy_tree();
    //println!("hierarchy_tree:\n{}", tree);

    let node_path: NodePath = "/".try_into().unwrap();
    let children = get_child_nodes(&*store, &node_path).unwrap();
    for child in children {
        println!("child_metadata: {:?}", child);
    }
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

fn find_zarr_elements() {
    todo!();
}

fn select_file(dir_path: &Path) -> PathResult {
    let paths: Vec<PathBuf> = find_files(dir_path.to_path_buf());

    let mut matching_name: HashMap<String, PathBuf> = HashMap::new();
    let mut options: Vec<String> = Vec::new();

    for path in paths {
        let path_name = path.file_name().unwrap().to_str().unwrap().to_string();
        options.push(path_name.clone());
        matching_name.insert(path_name, path);
    }

    // Add the option to go back to the parent directory
    options.push("Go back!".to_string());
    let parent_path = dir_path.parent().unwrap().to_path_buf();
    matching_name.insert("Go back!".to_string(), parent_path);

    // Add the option to exit the program
    options.push("Exit!".to_string());
    matching_name.insert("Exit!".to_string(), PathBuf::new());

    let ans = Select::new("Select a Directory or a Zarr file", options).prompt();
    let ans_name = ans.unwrap();
    let ans_path: PathBuf = matching_name.get(&ans_name).unwrap().clone();

    if ans_path.is_dir() && ans_name.ends_with(".zarr") {
        return PathResult::ZarrFile(ans_path);
    } else if ans_path.is_dir() {
        return PathResult::Directory(ans_path);
    } else if ans_name == "Exit!" {
        return PathResult::Exit;
    }

    PathResult::Error(Error)
}

fn main() {
    let mut path: PathBuf = PathBuf::from("/home/lcerrone/").canonicalize().unwrap();
    loop {
        let ans: PathResult = select_file(&path);
        match ans {
            PathResult::Directory(new_path) => {
                path = new_path;
            }
            PathResult::ZarrFile(zarr_path) => {
                println!("Zarr file selected: {:?}", zarr_path);
                WalkZarr(zarr_path);
            }
            PathResult::Exit => {
                println!("Exiting...");
                break;
            }
            PathResult::Error(err) => {
                println!("Error: {:?}", err);
            }
        }
    }
}
