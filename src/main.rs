extern crate git2;
extern crate libgpm;
extern crate toml;

use libgpm::ConfigurationFile;
use libgpm::file::{TomlExtension, ConfigurationFileExtension};
use libgpm::scan::{find_git_repositories, filter_hidden_repositories};
use std::path::{Path, PathBuf};

fn main() {
    let mut directories: Vec<String> = Vec::new();
    let path: PathBuf = PathBuf::from("/home/antonin/");
    find_git_repositories(&mut directories, &path);
    for repo in filter_hidden_repositories(&directories) {
        println!("{}", repo);
    }
    let toml_table = toml::Parser::parse_from_file(Path::new("/home/antonin/.gpm")).unwrap();
    println!("{:?}", toml_table);
    let encoding_str = ConfigurationFile::init();
    match encoding_str.save(Path::new("/home/antonin/.gpm")) {
        Ok(_) => println!("It works!"),
        Err(error) => println!("{}", error),
    }
}
