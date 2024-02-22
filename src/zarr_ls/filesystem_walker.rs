use super::common::{SelectionType, zarrfile_to_node, ZarrIdentifier, EXIT, BACK};
use super::common::{ActionSelect, LoopStep};
use indexmap::IndexMap;

use std::fs;
use std::path::PathBuf;

static ZARR_EXTENSION: &str = "zarr";


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

fn path_to_selection(path: PathBuf) -> SelectionType {
    let path_extension = match path.extension() {
        Some(ext) => ext.to_str().unwrap(),
        None => "",
    };

    if path_extension == ZARR_EXTENSION {
        let zarr_identifier = ZarrIdentifier::ZarrPath(path.clone());
        let node = zarrfile_to_node(&zarr_identifier);
        return SelectionType::Zarr(node);
    }

    if path.is_dir() {
        return SelectionType::Dir(path);
    }

    SelectionType::ExitWithError("This should not happen".to_string())
    
}

#[derive(Debug, Clone)]
pub struct FileWalker {
    current_path: PathBuf,
}


impl FileWalker {
    pub fn new(path: &PathBuf) -> Self {
        FileWalker { current_path: path.clone() }
    }

    pub fn get_options(&self) -> IndexMap<String, SelectionType> {
        let paths: Vec<PathBuf> = read_dir(self.current_path.clone());
        let mut matching_name: IndexMap<String, SelectionType> = IndexMap::new();
    
        for path in paths {
            let path_name = path.file_name().unwrap().to_str().unwrap().to_string();
            let current_selection = path_to_selection(path.clone());
            matching_name.insert(path_name, current_selection);
        }

        matching_name.insert(BACK.to_string(), SelectionType::Back);
        matching_name.insert(EXIT.to_string(), SelectionType::Exit);
        matching_name
    }

    fn parent(&mut self) -> FileWalker {
        let previous_path = self.current_path.parent().unwrap().to_path_buf();
        FileWalker::new(&previous_path)
    }
}