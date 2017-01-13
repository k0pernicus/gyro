extern crate git2;
extern crate libgpm;
extern crate rustc_serialize;
extern crate toml;

use libgpm::ConfigurationFile;
use libgpm::configuration::{ConfigureContent, Entry, EntryCategory};
use libgpm::file::{TomlExtension, ConfigurationFileExtension};
use libgpm::git::Repo;
use libgpm::scan::{find_git_repositories, filter_hidden_repositories};
use rustc_serialize::Encodable;
use std::path::{Path, PathBuf};

fn main() {
    let mut directories: Vec<String> = Vec::new();
    let path: PathBuf = PathBuf::from("/home/antonin/");
    find_git_repositories(&mut directories, &path);
    for repo in filter_hidden_repositories(&directories) {
        println!("{}", Repo(git2::Repository::open(repo).unwrap()));
    }
    let mut toml_table = toml::Parser::parse_from_file(Path::new("/home/antonin/.gpm")).unwrap();
    println!("PROCESSING...");
    match toml_table.transfer_entry("Test", &EntryCategory::Watched, &EntryCategory::Ignored) {
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
