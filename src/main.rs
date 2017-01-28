#[macro_use]
extern crate clap;
extern crate libgpm;
extern crate rustc_serialize;
extern crate toml;

pub mod commands;

use clap::{Arg, App, SubCommand};
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
use std::process::exit;

/// A type that represents a binary tree map of groups - each group represents a couple (String
/// type, Vector of String types)
type GVec = BTreeMap<String, Vec<String>>;
/// A type that represents a "list" of git repositories that have to be ignored
type IVec = ConfigurationContent;
/// A type that represents a "list" of git repositories that have to be watched
type WVec = ConfigurationContent;

fn main() {

    // Command line arguments
    let matches = App::new(commands::PRG_NAME)
        .version(crate_version!())
        .author("A. Carette <antonin@carette.xyz>")
        .about("Your Git Project Manager")
        .arg(Arg::with_name(commands::RESET_FLAG)
            .short("r")
            .long("reset")
            .help("Reset the entire configuration file to the default values"))
        .subcommand(SubCommand::with_name(commands::DIFF_SUBCMD)
            .author("A. Carette <antonin@carette.xyz>")
            .about("Get the difference between current state and new local git repositories \
                    unfollowed"))
        .subcommand(SubCommand::with_name(commands::STORE_SUBCMD)
            .author("A. Carette <antonin@carette.xyz>")
            .about("Manage storing behaviours, for new git repositories")
            .arg(Arg::with_name(commands::STORE_SUBCMD_DEFAULT_FLAG)
                .short("d")
                .long("default")
                .help("Default 'location' of new git repositories")
                .takes_value(true)
                .possible_values(&[IGNORED_ENTRY_NAME, WATCHED_ENTRY_NAME])
                .default_value(WATCHED_ENTRY_NAME)))
        .get_matches();

    // Get the user home directory, and push the name of the gpm configuration file
    let mut configuration_file_path = match env::home_dir() {
        Some(home_dir) => PathBuf::from(home_dir),
        None => panic!("Home directory canno't be reached"),
    };
    configuration_file_path.push(CONFIGURATION_FILE_NAME);
    let configuration_file_path_str = configuration_file_path.to_str().unwrap();
    // Get the TOML table, or init a new one
    let mut toml_table = match (toml::Parser::parse_from_file(configuration_file_path.as_path()),
                                matches.is_present("reset")) {
        (Some(toml_table), false) => toml_table,
        (_, true) => {
            println!("[WARNING] Reseting your default configuration file...");
            ConfigurationFile::init().toml
        }
        (None, _) => {
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

    if matches.is_present("diff") {
        for gitrepo in &filtered_git_repositories {
            let gitrepo_name = gitrepo.split("/").last().unwrap();
            let gitrepo_name_s = String::from(gitrepo_name);
            if !(vec_watched.contains(&gitrepo_name_s) || vec_ignored.contains(&gitrepo_name_s)) {
                println!("New repository to save: {}", gitrepo);
            }
            exit(0);
        }
    }

    let entry_category =
        if matches.subcommand_matches("store").unwrap().value_of("default").unwrap() ==
           WATCHED_ENTRY_NAME {
            EntryCategory::Watched
        } else {
            EntryCategory::Ignored
        };
    for gitrepo in &filtered_git_repositories {
        let gitrepo_name = gitrepo.split("/").last().unwrap();
        let gitrepo_name_s = String::from(gitrepo_name);
        if !(vec_watched.contains(&gitrepo_name_s) || vec_ignored.contains(&gitrepo_name_s)) {
            match toml_table.add_entry(gitrepo_name,
                                       &mut Entry::new(gitrepo_name, gitrepo),
                                       &entry_category) {
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
