use GIT_DIR_NAME;
use std::fs;
use std::io::prelude::*;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::{DirEntry, WalkDir};

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

trait GitTools {
    ///
    /// Method to know if a given entry is a git repository
    ///
    fn is_git_repository(&self) -> bool;
}

impl GitTools for DirEntry {
    ///
    /// This method returns a boolean - true if the DirEntry reference is a git repository, else
    /// false
    ///
    fn is_git_repository(&self) -> bool {
        if fs::metadata(self.path()).unwrap().is_dir() {
            // If the DirEntry is a directory, it returns if the DirEntry metadata is equals to GIT_DIR_NAME
            return self.file_name().to_str().unwrap() == GIT_DIR_NAME;
        }
        // If the DirEntry is not a directory, return false
        return false;
    }
}

///
/// Function to scan all directories from a single directory.
///
/// This function update a mutable vector of String, which represents each path for each git
/// path for a given git repository.
///
pub fn find_git_repositories(git_path: &mut Vec<String>, directory: &PathBuf) {
    print!("Scanning repository from {:?} to find git repositories... (this can take a while) ",
           directory);
    io::stdout().flush().ok().expect("Could not flush stdout");
    // Get all entries from the PathBuf given as parameter, filter and follow links
    for entry in WalkDir::new(directory).follow_links(true).into_iter().filter_map(|e| e.ok()) {
        if entry.is_git_repository() {
            // Get the parent node
            let entry_p = entry.path().parent().unwrap().clone();
            // Puth the parent path name in the list of git repositories found
            git_path.push(entry_p.to_str().unwrap().to_string());
        }
    }
    println!("Ok!");
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
