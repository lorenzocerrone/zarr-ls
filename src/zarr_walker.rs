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

enum ZarrIdentifier {
    ZarrPath(PathBuf),
    ZarrURL(String),
}

fn node_from_zarrfile_path(path: PathBuf) -> Node {
    let store = FilesystemStore::new(path).unwrap();
    Node::new(&store, "/").unwrap()
}

fn node_from_zarrfile_url(url: String) -> Node {
    panic!("URL not implemented yet")
}


fn zarrfile_to_node(zarr_identifier: ZarrIdentifier) -> Node {
    match zarr_identifier {
        ZarrIdentifier::ZarrPath(path) => zarr_node_from_path(path),
        ZarrIdentifier::ZarrURL(url) => zarr_node_from_url(url),
    }
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

struct ZarrWalker {
    current_node: Node,
    visited_nodes: LinkedList<Node>,
}


impl ZarrWalker {
    pub fn new(zarr_identifier: ZarrIdentifier) -> Self {
        let node = zarrfile_to_node(zarr_identifier);
        ZarrWalker {
            current_node: node,
            visited_nodes: LinkedList::new(),
        }
    }

    pub fn get_options(&self) -> IndexMap<String, Node> {
        let mut options = IndexMap::new();
        for (name, node) in self.current_node.children() {
            let child_name = format_zarr_option_name(child.clone());
            let current_selection = CurrentSelection::ZarrFile(child.clone());
            options.insert(name.to_string(), node);
        }
        options
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

