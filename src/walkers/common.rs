





use std::path::PathBuf;

use zarrs::node::{Node};

use zarrs::storage::store::FilesystemStore;

static ZARR_GROUP_HEADER: &str = "Zarr Group: ";
static ZARR_ARRAY_HEADER: &str = "Zarr Array: ";
static ZARR_EXTENSION: &str = "zarr";
static EXIT: &str = "Exit!";
static BACK: &str = "..";


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

#[derive(Debug, Clone)]
pub enum SelectionType {
    Dir(PathBuf),
    Zarr(Node),
    ExitWithError(String),
    Exit,
}