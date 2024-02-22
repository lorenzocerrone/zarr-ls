use crate::zarr_ls::common::{ActionSelect, ZARR_ARRAY_HEADER, ZARR_GROUP_HEADER};
use std::path::PathBuf;

use indexmap::IndexMap;
use zarrs::array::ArrayMetadata;

use zarrs::group::GroupMetadata;

use zarrs::node::{Node, NodeMetadata};
use zarrs::storage::store::FilesystemStore;

pub enum ZarrIdentifier {
    ZarrPath(PathBuf),
    ZarrURL(String),
    ZarrNode(Node),
}

fn node_from_zarrfile_path(path: &PathBuf) -> Node {
    let store = FilesystemStore::new(path).unwrap();
    Node::new(&store, "/").unwrap()
}

fn node_from_zarrfile_url(_url: &String) -> Node {
    panic!("URL not implemented yet")
}

pub fn zarrfile_to_node(zarr_identifier: &ZarrIdentifier) -> Node {
    match zarr_identifier {
        ZarrIdentifier::ZarrPath(path) => node_from_zarrfile_path(path),
        ZarrIdentifier::ZarrURL(url) => node_from_zarrfile_url(url),
        ZarrIdentifier::ZarrNode(node) => node.clone(),
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
                format!(
                    "\n{}",
                    serde_json::to_string_pretty(&array_metadata).unwrap()
                )
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

fn format_zarr_option_name(node: &Node) -> String {
    let infos = match node.metadata() {
        NodeMetadata::Array(_) => format_array_infos(node, true),
        NodeMetadata::Group(_) => format_group_infos(node),
    };
    infos
}

#[derive(Debug, Clone)]
pub struct ZarrNode {
    node: Node,
    parent_directory: Option<PathBuf>,
}

impl ZarrNode {
    pub fn new(zarr_identifier: &ZarrIdentifier) -> Self {
        let node = zarrfile_to_node(zarr_identifier);
        let parent_directory = match zarr_identifier {
            ZarrIdentifier::ZarrPath(path) => path.parent().map(|path| path.to_path_buf()),
            _ => None,
        };
        ZarrNode {
            node,
            parent_directory,
        }
    }

    pub fn new_from_node(node: &Node, parent_directory: Option<PathBuf>) -> Self {
        ZarrNode {
            node: node.clone(),
            parent_directory,
        }
    }

    pub fn get_options(&self) -> IndexMap<String, ActionSelect> {
        let mut options = IndexMap::new();

        for node in self.node.children() {
            let child_name = format_zarr_option_name(node);
            let zarr_node = ZarrNode::new_from_node(node, self.parent_directory.clone());

            let selection = ActionSelect::Zarr(zarr_node);
            options.insert(child_name, selection);
        }
        options
    }
}
