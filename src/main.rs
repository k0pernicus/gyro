extern crate git2;
extern crate libgpm;
extern crate rustc_serialize;
extern crate toml;

use libgpm::{ConfigurationContent, ConfigurationFile, CONFIGURATION_FILE_NAME, GROUPS_ENTRY_NAME,
             IGNORED_ENTRY_NAME, WATCHED_ENTRY_NAME};
use libgpm::configuration::{ConfigureContent, Entry, EntryCategory};
use libgpm::file::{TomlExtension, ConfigurationFileExtension};
use libgpm::git::Repo;
use libgpm::scan::{find_git_repositories, filter_hidden_repositories};
use rustc_serialize::Encodable;
use std::collections::BTreeMap;
use std::env;
use std::path::{Path, PathBuf};

/// A type that represents a binary tree map of groups - each group represents a couple (String
/// type, Vector of String types)
type GVec = BTreeMap<String, Vec<String>>;
/// A type that represents a "list" of git repositories that have to be ignored
type IVec = ConfigurationContent;
/// A type that represents a "list" of git repositories that have to be watched
type WVec = ConfigurationContent;

fn test() {
    let toml_table = toml::Parser::parse_from_file(Path::new("/home/antonin/.gpm")).unwrap();
    // TODO: Create two vectors: a watched vector (typed WVec), an ignored vector (type IVec), a group vector (type GVec) - that contains Entry objects (excepts the last one)
    // TODO: Fill those vectors with (key, value) from the TOML table variable
    // TODO: Display them!
    let mut vec_watched: Vec<String> = Vec::new();
    let mut vec_ignored: Vec<String> = Vec::new();
    for (key, value) in toml_table.iter() {
        if key.starts_with(WATCHED_ENTRY_NAME) {
            println!("{} -> {}", key, value);
            vec_watched.push(key.clone());
        }
        if key.starts_with(IGNORED_ENTRY_NAME) {
            println!("{} -> {}", key, value);
            vec_ignored.push(key.clone());
        }
    }
    let test = toml_table.get("watched.Test").unwrap().as_table().unwrap();
    println!("{}", test.get("name").unwrap());
}

fn main() {
    // Get the user home directory, and push the name of the gpm configuration file
    let mut configuration_file_path = match env::home_dir() {
        Some(home_dir) => PathBuf::from(home_dir),
        None => panic!("Home directory canno't be reached"),
    };
    configuration_file_path.push(CONFIGURATION_FILE_NAME);
    let configuration_file_path_str = configuration_file_path.to_str().unwrap();
    // Get the TOML table, or init a new one
    let mut toml_table = match toml::Parser::parse_from_file(configuration_file_path.as_path()) {
        Some(toml_table) => toml_table,
        None => {
            println!("[WARNING] Cannot find the current configuration of the configuration \
                      file...\n[WARNING] Declaration in {}",
                     configuration_file_path_str);
            ConfigurationFile::init().toml
        }
    };
    // Get local git path directories
    let mut gitpath_directories: Vec<String> = Vec::new();
    find_git_repositories(&mut gitpath_directories, &env::home_dir().unwrap());
    // Get git repositories that are not in an hidden path
    let filtered_git_repositories = filter_hidden_repositories(&gitpath_directories);
    // Declare && initialize repositories vectors
    let mut vec_watched = Vec::new();
    let mut vec_ignored = Vec::new();
    // Store watched and ignored git path repositories, from the configuration file
    for (key, value) in toml_table.iter() {
        if key.starts_with(WATCHED_ENTRY_NAME) && (key != WATCHED_ENTRY_NAME) {
            vec_watched.push(key.clone());
        }
        if key.starts_with(IGNORED_ENTRY_NAME) && (key != IGNORED_ENTRY_NAME) {
            vec_ignored.push(key.clone());
        }
    }
    for gitrepo in &filtered_git_repositories {
        let gitrepo_name = gitrepo.split("/").last().unwrap();
        let gitrepo_name_s = String::from(gitrepo_name);
        if !(vec_watched.contains(&gitrepo_name_s) || vec_ignored.contains(&gitrepo_name_s)) {
            match toml_table.add_entry(gitrepo_name,
                                       &mut Entry::new(gitrepo_name, gitrepo),
                                       &EntryCategory::Watched) {
                Ok(_) => {
                    println!("{} has been added to {}",
                             gitrepo_name,
                             configuration_file_path_str)
                }
                Err(error) => println!("{:?}", error),
            }
        }
    }
    let mut encoding_str = ConfigurationFile::init();
    match toml_table.encode(&mut encoding_str) {
        Ok(_) => {
            match encoding_str.save(Path::new(configuration_file_path.as_path())) { 
                Ok(_) => println!("It works!"),
                Err(error) => println!("{}", error),
            }
        }
        Err(error) => println!("{:?}", error),
    }
}
