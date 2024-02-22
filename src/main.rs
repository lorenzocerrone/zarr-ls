mod zarr_ls {
    pub mod common;
    pub mod filesystem_utils;
    pub mod walkers;
    pub mod zarr_utils;
}

use zarr_ls::common::ActionSelect;
use zarr_ls::filesystem_utils::DirNode;
use zarr_ls::walkers::Walker;

fn main() {
    let initial_selection = ActionSelect::Dir(DirNode::new(&std::env::current_dir().unwrap()));
    let mut walker = Walker::new(&initial_selection);
    walker.run();
}
