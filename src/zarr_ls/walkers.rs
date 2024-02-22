use super::common::{ActionSelect, LoopStep};
use indexmap::IndexMap;
use inquire::Select;
use zarrs::array::ArrayMetadata;

use zarrs::group::GroupMetadata;

use std::collections::LinkedList;
use std::path::PathBuf;

use zarrs::node::{Node, NodeMetadata};

static ZARR_GROUP_HEADER: &str = "Zarr Group: ";
static ZARR_ARRAY_HEADER: &str = "Zarr Array: ";
static ZARR_EXTENSION: &str = "zarr";

#[derive(Debug, Clone)]
pub struct Walker {
    visited_nodes: LinkedList<ActionSelect>,
}

pub trait GenerateOptions<T> {
    type Item;
    fn get_options(&self) -> Self::Item;
}

impl Walker {
    pub fn new(root: &ActionSelect) -> Self {
        let mut visited_nodes = LinkedList::new();
        visited_nodes.push_back(root.clone());

        Walker {
            visited_nodes: visited_nodes,
        }
    }

    fn current_selection(&self) -> &ActionSelect {
        self.visited_nodes.back().unwrap()
    }

    fn walk_back(&mut self) -> &ActionSelect {
        self.visited_nodes.pop_back().unwrap();
        self.visited_nodes.back().unwrap()
    }

    fn walk_forward(&mut self, next: ActionSelect) {
        self.visited_nodes.push_back(next);
    }

    fn get_options(&self) -> Result<IndexMap<String, ActionSelect>, String> {
        let current_selection = self.current_selection().clone();
        let mut index_options: IndexMap<String, ActionSelect> = match current_selection {
            ActionSelect::Dir(path) => todo!(),
            ActionSelect::Zarr(node) => todo!(),
            _ => return Err("This should not happen".to_string()),
        };

        index_options.insert("Back".to_string(), ActionSelect::Back);
        index_options.insert("Exit".to_string(), ActionSelect::Exit);
        Ok(index_options)
    }

    fn build_menu_and_select(&self) -> ActionSelect {
        let matching_options = self.get_options().unwrap();
        let options: Vec<String> = matching_options.keys().cloned().collect();
        let selection = Select::new("Select a directory or zarr file", options).prompt();
        let selected_option_name = selection.unwrap();
        let selected_option = matching_options.get(&selected_option_name).unwrap();
        selected_option.clone()
    }

    fn step(&mut self) -> LoopStep {
        let next_selection = self.build_menu_and_select();
        match next_selection {
            ActionSelect::Dir(path) => {
                let next = !todo!();
                self.walk_forward(next);
                LoopStep::Continue
            }
            ActionSelect::Zarr(node) => {
                let next = !todo!();
                self.walk_forward(next);
                LoopStep::Continue
            }
            ActionSelect::Back => {
                let next = self.walk_back();
                LoopStep::Continue
            }
            ActionSelect::Error(message) => LoopStep::Error(message),
            ActionSelect::Exit => LoopStep::Exit,
        }
    }

    pub fn run(&mut self) {
        loop {
            let step = self.step();
            match step {
                LoopStep::Continue => continue,
                LoopStep::Error(message) => {
                    println!("{}", message);
                    continue;
                }
                LoopStep::Exit => {
                    println!("Exiting");
                    break;
                }
            }
        }
    }
}
