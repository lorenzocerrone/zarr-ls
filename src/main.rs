use indexmap::IndexMap;
use inquire::Select;
use zarrs::array::ArrayMetadata;

use zarrs::group::GroupMetadata;

use std::collections::LinkedList;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use zarrs::node::{Node, NodeMetadata};
use std::env;
use clap::{Error, Parser};

use zarrs::storage::store::FilesystemStore;

static ZARR_GROUP_HEADER: &str = "Zarr Group: ";
static ZARR_ARRAY_HEADER: &str = "Zarr Array: ";
static ZARR_EXTENSION: &str = "zarr";
static EXIT: &str = "Exit!";
static BACK: &str = "..";

#[derive(Parser)]
struct Cli {
    path: Option<std::path::PathBuf>
}


#[derive(Debug, Clone)]
enum CurrentSelection {
    Directory(PathBuf),
    ZarrFile(Node),
    Exit,
    Back(Option<PathBuf>),
}

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
    
    let parent_path = match path.parent(){
        Some(path) => Some(path.to_path_buf()),
        None => None
    };
    
    matching_name.insert(BACK.to_string(), CurrentSelection::Back(parent_path));
    matching_name.insert(EXIT.to_string(), CurrentSelection::Exit);
    matching_name
}

fn zarr_node_from_path(path: PathBuf) -> Node {
    let store = FilesystemStore::new(path).unwrap();
    Node::new(&store, "/").unwrap()
}

fn format_array_infos(node: &Node, verbose: bool) -> String {
    let name = node.path();
    let metadata: &ArrayMetadata = match node.metadata() {
        NodeMetadata::Array(array_metadata) => array_metadata,
        NodeMetadata::Group(_) => panic!("This should not be a group"),
    };
    let infos: String = match metadata {
        ArrayMetadata::V3(array_metadata) => {
            let shape = &array_metadata.shape;
            let dtype = array_metadata.data_type.name();

            let more_info = if verbose {
                format!("\n{}", serde_json::to_string_pretty(&array_metadata).unwrap())
            } else {
                "".to_string()
            };

            format!("{:?} - {} {}", shape, dtype, more_info)
        }
    };

    if verbose {
        format!("{}{}{}", ZARR_ARRAY_HEADER, name, infos)
    } else {
        format!("{}{}", name, infos)
    }
}

fn format_group_infos(node: &Node) -> String {
    let name = node.path();
    let child_headers = format!("\n  {:-<1$}>", "", ZARR_GROUP_HEADER.len() - 1);
    let metadata: &GroupMetadata = match node.metadata() {
        NodeMetadata::Group(group_metadata) => group_metadata,
        NodeMetadata::Array(_) => panic!("This should not be an array"),
    };

    let infos: String = match metadata {
        GroupMetadata::V3(_group_metadata) => {
            let num_children = node.children().len();
            let mut child_infos: String = "".to_string();

            for child in node.children() {
                let child_metadata = child.metadata();
                let _child_infos = match child_metadata {
                    NodeMetadata::Array(_) => format_array_infos(child, false),
                    NodeMetadata::Group(_) => format_group_infos(child),
                };

                child_infos.push_str(&child_headers);
                child_infos.push_str(&_child_infos);
            }
            format!(
                "{}{} - contains {} elements {}",
                ZARR_GROUP_HEADER, name, num_children, child_infos
            )
        }
    };
    infos
}

fn format_zarr_option_name(node: Node) -> String {
    let infos = match node.metadata() {
        NodeMetadata::Array(_) => format_array_infos(&node, true),
        NodeMetadata::Group(_) => format_group_infos(&node),
    };
    infos
}

fn find_zarr_options(node: Node) -> IndexMap<String, CurrentSelection> {
    let mut matching_name: IndexMap<String, CurrentSelection> = IndexMap::new();
    for child in node.children() {
        let child_name = format_zarr_option_name(child.clone());
        let current_selection = CurrentSelection::ZarrFile(child.clone());
        matching_name.insert(child_name, current_selection);
    }

    matching_name.insert(BACK.to_string(), CurrentSelection::Back(None));
    matching_name.insert(EXIT.to_string(), CurrentSelection::Exit);

    matching_name
}

fn build_menu(current_path: CurrentSelection) -> CurrentSelection {
    let matching_selections: IndexMap<String, CurrentSelection> = match current_path {
        CurrentSelection::Directory(dir_path) => find_files_options(dir_path),
        CurrentSelection::ZarrFile(zarr_path) => find_zarr_options(zarr_path),
        CurrentSelection::Exit => panic!("Exit should not be a valid option"),
        CurrentSelection::Back(_) => panic!("Back should not be a valid option"),
    };

    let options: Vec<String> = matching_selections.keys().cloned().collect();
    let selection = Select::new("Select a directory or zarr file", options).prompt();
    let selected_option_name = selection.unwrap();
    matching_selections
        .get(&selected_option_name)
        .unwrap()
        .clone()
}

fn main() {
    // Prints each argument on a separate line
    let args: Cli = Cli::parse();
    let path = match args.path {
        Some(path) => path,
        None => env::current_dir().unwrap(),
    };

    let mut selections: LinkedList<CurrentSelection> = LinkedList::new();

    let current_selection = path_to_selection(path.clone());
    selections.push_back(current_selection);

    loop {
        let current_selection = selections.back();
        let current_selection = match current_selection {
            Some(selection) => selection,
            None => panic!("No selection found"),
        };

        let next_selection = build_menu(current_selection.clone());

        match next_selection {
            CurrentSelection::Exit => break,
            CurrentSelection::Back(previous_path) => {

                match previous_path {
                    Some(path) => {
                        selections.push_back(CurrentSelection::Directory(path));
                    }
                    None => {
                        if selections.len() == 1 {
                            continue;
                        }

                        selections.pop_back();
                    }
                }
            }
            _ => {
                selections.push_back(next_selection);
            }
        }
    }
}
