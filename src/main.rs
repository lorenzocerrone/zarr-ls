pub mod walkers {
    pub mod filesystem_walker;
    pub mod zarr_walker;
    pub mod common;
}

use walkers::{filesystem_walker::FileWalker, zarr_walker};
use walkers::zarr_walker::ZarrWalker;
use walkers::common::{self, SelectionType};

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

static EXIT: &str = "Exit!";
static BACK: &str = "..";



#[derive(Parser)]
struct Cli {
    path: Option<std::path::PathBuf>
}

#[derive(Debug, Clone)]
pub enum Walker {
    Dir(FileWalker),
    Zarr(ZarrWalker)
}

fn build_menu(walker: &Walker) -> SelectionType {

    let matching_options: IndexMap<String, SelectionType> = match walker {
        Walker::Dir(file_walker) => file_walker.get_options(),
        Walker::Zarr(zarr_walker) => zarr_walker.get_options(),
    };

    let options: Vec<String> = matching_options.keys().cloned().collect();
    let selection = Select::new("Select a directory or zarr file", options).prompt();
    let selected_option_name = selection.unwrap();
    let selected_option = matching_options.get(&selected_option_name).unwrap();
    return selected_option.clone();   
}

fn main() {
    // Prints each argument on a separate line
    let args: Cli = Cli::parse();
    let path = match args.path {
        Some(path) => path,
        None => env::current_dir().unwrap(),
    };

    let walker = Walker::Dir(FileWalker::new(&path));

    loop {
        let current_selection = build_menu(&walker);
        let walker = match current_selection {
            SelectionType::Dir(path) => {
                Walker::Dir(FileWalker::new(&path))
            },
            SelectionType::Zarr(node) => {
                match walker {
                    Walker::Dir(_) => Walker::Zarr(ZarrWalker::new(common::ZarrIdentifier::ZarrNode(node))),
                    Walker::Zarr(ref zarr_walker) => {
                        let next_walker = zarr_walker.clone().next(node);
                        Walker::Zarr(next_walker)
                    },    
                }
            },
            SelectionType::ExitWithError(error) => {
                println!("{}", error);
                break;
            },
            SelectionType::Exit => {
                break;
            },

        };
    }
}
