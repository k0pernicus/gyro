use std::fs;
use std::path::{Path, PathBuf};

use walkdir::{DirEntry, WalkDir};

///
/// Static variable to get the name of the git main directory
///
static GIT_DIR_NAME: &'static str = ".git";

trait HiddenPath {
    ///
    /// Method to know if a given file or directory is contained in an hidden directory
    ///
    fn is_in_hidden_dir(&self) -> bool;
}

impl HiddenPath for Path {
    ///
    /// This method returns a boolean - true if the given path is contained in an hidden directory,
    /// else false
    ///
    fn is_in_hidden_dir(&self) -> bool {
        if self.parent() == None {
            return false;
        }
        let mut iterator = self.iter();
        loop {
            match iterator.next() {
                Some(slice_p) => {
                    if slice_p.to_str().unwrap().starts_with('.') {
                        return true;
                    }
                }
                None => break,
            }
        }
        return false;
    }
}

///
/// Fonction to know if a directory path is a git repository.
///
/// Is a git repository if the path indicates a directory, and this directory is a git subdirectory.
///
pub fn scan_single_directory(directory: &DirEntry) -> bool {
    if fs::metadata(directory.path()).unwrap().is_dir() {
        // If the DirEntry is a directory, it returns if the DirEntry metadata is equals to GIT_DIR_NAME
        return directory.file_name().to_str().unwrap() == GIT_DIR_NAME;
    }
    // If the DirEntry is not a directory, return false
    return false;
}

///
/// Function to scan all directories from a single directory.
///
/// This function update a mutable vector of String, which represents each path for each git
/// path for a given git repository.
///
pub fn scan_repositories(git_path: &mut Vec<String>, directory: &PathBuf) {
    // Get all entries from the PathBuf given as parameter, filter and follow links
    for entry in WalkDir::new(directory).follow_links(true).into_iter().filter_map(|e| e.ok()) {
        if scan_single_directory(&entry) {
            // Get the parent node
            let entry_p = entry.path().parent().unwrap().clone();
            // Puth the parent path name in the list of git repositories found
            git_path.push(entry_p.to_str().unwrap().to_string());
        }
    }
}

///
/// Function to remove each hidden file from a String vector.
///
/// This function returns a vector of String references, which each reference represents a path repository.
///
pub fn filter_hidden_repositories<'a>(git_repositories: &'a Vec<String>) -> Vec<&'a String> {
    git_repositories.iter()
        .filter(|path_string| {
            let current_path = Path::new(path_string);
            !current_path.is_in_hidden_dir()
        })
        .collect::<Vec<&String>>()
}
