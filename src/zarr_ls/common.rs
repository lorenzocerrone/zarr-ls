use super::filesystem_utils::DirNode;
use super::zarr_utils::ZarrNode;

use indexmap::IndexMap;
use inquire::Select;
use zarrs::array::ArrayMetadata;

use zarrs::group::GroupMetadata;

use std::collections::LinkedList;
use std::path::PathBuf;

use zarrs::node::{Node, NodeMetadata};

pub static ZARR_GROUP_HEADER: &str = "Zarr Group: ";
pub static ZARR_ARRAY_HEADER: &str = "Zarr Array: ";
pub static ZARR_EXTENSION: &str = "zarr";
pub static EXIT: &str = "Exit!";
pub static BACK: &str = "..";

#[derive(Debug, Clone)]
pub enum ActionSelect {
    Zarr(ZarrNode),
    Dir(DirNode),
    Back,
    Error(String),
    Exit,
}

pub enum LoopStep {
    Continue,
    Error(String),
    Exit,
}

pub trait GenerateOptions<T> {
    type Item;
    fn get_options(&self) -> Self::Item;
}
