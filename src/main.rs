extern crate git2;
extern crate libgpm;
extern crate rustc_serialize;
extern crate toml;

use libgpm::{ConfigurationContent, ConfigurationFile, GROUPS_ENTRY_NAME, IGNORED_ENTRY_NAME, WATCHED_ENTRY_NAME};
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

fn add_all() {
    let mut directories: Vec<String> = Vec::new();
    let path = match env::home_dir() {
        Some(home_dir) => home_dir,
        None => panic!("Can not set your home directory!"),
    };
    find_git_repositories(&mut directories, &path);
    for repo in filter_hidden_repositories(&directories) {
        println!("{}", Repo(git2::Repository::open(repo).unwrap()));
    }
    let mut toml_table = toml::Parser::parse_from_file(Path::new("/home/antonin/.gpm")).unwrap();
    println!("PROCESSING...");
    match toml_table.add_entry("Test", &mut Entry::new("Test", "TestPath"), &EntryCategory::Watched) {
        Ok(_) => println!("{:?}", toml_table),
        Err(error) => println!("{:?}", error),
    }
    match toml_table.add_entry("Test2", &mut Entry::new("Test2", "Test2Path"), &EntryCategory::Watched) {
        Ok(_) => println!("{:?}", toml_table),
        Err(error) => println!("{:?}", error),
    }
    match toml_table.add_entry("Test3", &mut Entry::new("Test3", "Test3Path"), &EntryCategory::Ignored) {
        Ok(_) => println!("{:?}", toml_table),
        Err(error) => println!("{:?}", error),
    }
    match toml_table.add_entry("Test4", &mut Entry::new("Test4", "Test4Path"), &EntryCategory::Watched) {
        Ok(_) => println!("{:?}", toml_table),
        Err(error) => println!("{:?}", error),
    }
    let mut encoding_str = ConfigurationFile::init();
    match toml_table.encode(&mut encoding_str) {
        Ok(_) => { match encoding_str.save(Path::new("/home/antonin/.gpm")) {
                        Ok(_) => println!("It works!"),
                        Err(error) => println!("{}", error),
                }
        },
        Err(error) => println!("{:?}", error),
    }
}

fn main() {
    let toml_table = toml::Parser::parse_from_file(Path::new("/home/antonin/.gpm")).unwrap();
    // TODO: Create two vectors: a watched vector (typed WVec), an ignored vector (type IVec), a group vector (type GVec) - that contains Entry objects (excepts the last one)
    // TODO: Fill those vectors with (key, value) from the TOML table variable
    // TODO: Display them!
    let mut vec_watched : Vec<String> = Vec::new();
    let mut vec_ignored : Vec<String> = Vec::new();
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
