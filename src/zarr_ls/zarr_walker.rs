use indexmap::IndexMap;
use super::common::{ZarrIdentifier, zarrfile_to_node, SelectionType};
use super::filesystem_walker::FileWalker;
use zarrs::array::ArrayMetadata;

use zarrs::group::GroupMetadata;

use std::collections::LinkedList;
use std::path::PathBuf;

use zarrs::node::{Node, NodeMetadata};

static ZARR_GROUP_HEADER: &str = "Zarr Group: ";
static ZARR_ARRAY_HEADER: &str = "Zarr Array: ";
static ZARR_EXTENSION: &str = "zarr";pub fn get_options(&self) -> IndexMap<String, SelectionType> {
    let mut options = IndexMap::new();
    
    for node in self.current_node.children() {
        let child_name = format_zarr_option_name(node.clone());
        
        let selection = SelectionType::Zarr(node.clone());
        options.insert(child_name, selection);
    }
    options
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
e,
                self.parent_directory.clone()
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

#[derive(Debug, Clone)]
pub struct ZarrWalker {
    current_node: Node,
    visited_nodes: LinkedList<Node>,
    parent_directory: Option<PathBuf>,
}

enum MaybeZarrWalker {
    Zarr(ZarrWalker),
    File(FileWalker),
    EXIT
}


impl ZarrWalker {
    pub fn new(zarr_identifier: ZarrIdentifier) -> Self {
        let node = zarrfile_to_node(&zarr_identifier);

        let parent_directory = match zarr_identifier {
            ZarrIdentifier::ZarrPath(path) => path.parent().map(|path| path.to_path_buf()),
            _ => None,
            };
        
        ZarrWalker {
            current_node: node,
            visited_nodes: LinkedList::new(),
            parent_directory,
        }
    }

    pub fn next(self, next_node: Node) -> ZarrWalker {
        let mut visited_nodes = self.visited_nodes;
        visited_nodes.push_back(self.current_node.clone());

        ZarrWalker {
            current_node: next_node,
            visited_nodes,
            parent_directory: self.parent_directory,
        }
    }

    pub fn get_options(&self) -> IndexMap<String, SelectionType> {
        let mut options = IndexMap::new();
        
        for node in self.current_node.children() {
            let child_name = format_zarr_option_name(node.clone());
            
            let selection = SelectionType::Zarr(node.clone());
            options.insert(child_name, selection);
        }
        options
    }

    pub fn parent(& self) -> MaybeZarrWalker{
        if self.visited_nodes.is_empty() {
            return match &self.parent_directory {
                Some(parent_directory) => MaybeZarrWalker::File(FileWalker::new(parent_directory)),
                None => MaybeZarrWalker::EXIT,
            }
        }

        let previous_node = self.visited_nodes.pop_back().unwrap();
        MaybeZarrWalker::Zarr(ZarrWalker {
            current_node: previous_node,
            visited_nodes: self.visited_nodes.clone(),
            parent_directory: self.parent_directory.clone(),
        })
    }
}

