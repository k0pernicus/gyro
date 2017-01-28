#[macro_use]
extern crate clap;
extern crate libgpm;
extern crate rustc_serialize;
extern crate toml;

pub mod commands;

use libgpm::{ConfigurationContent, ConfigurationFile, CONFIGURATION_FILE_NAME, BODY_ENTRY_NAME,
             GROUPS_ENTRY_NAME, IGNORED_ENTRY_NAME, WATCHED_ENTRY_NAME};
use libgpm::configuration::{ConfigureContent, Entry, EntryCategory};
use libgpm::file::{TomlExtension, ConfigurationFileExtension};
use libgpm::git;
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

fn get_configuration_file_content(configuration_file_path: &Path,
                                  reset_configuration_file: bool)
                                  -> ConfigurationContent {
    match (toml::Parser::parse_from_file(configuration_file_path), reset_configuration_file) {
        (Some(toml_table), false) => toml_table,
        (_, true) => {
            println!("[WARNING] Reseting your default configuration file...");
            ConfigurationFile::init().toml
        }
        (None, _) => {
            println!("[WARNING] Cannot find the current configuration of the configuration \
                      file...\n[WARNING] Declaration in {}",
                     configuration_file_path.to_str().unwrap());
            ConfigurationFile::init().toml
        }
    }
}

fn main() {

    // Command line arguments
    let matches = commands::get_program_args();

    if (!matches.is_present(commands::RESET_FLAG) && !matches.is_present(commands::SCAN_SUBCMD) &&
        !matches.is_present(commands::STATUS_SUBCMD) &&
        !matches.is_present(commands::OVERRIDE_SUBCMD)) {
        println!("{}", matches.usage());
        println!("\nYou can learn more about {} using {} --help !",
                 commands::PRG_NAME,
                 commands::PRG_NAME);
        exit(0);
    }

    // Get the user home directory, and push the name of the gpm configuration file
    let mut configuration_file_path = match env::home_dir() {
        Some(home_dir) => PathBuf::from(home_dir),
        None => panic!("Home directory canno't be reached"),
    };
    configuration_file_path.push(CONFIGURATION_FILE_NAME);
    // Get the TOML table, or init a new one
    let mut toml_table: ConfigurationContent =
        get_configuration_file_content(configuration_file_path.as_path(),
                                       matches.is_present(commands::RESET_FLAG));

    let toml_table_type = toml::Value::Table(toml_table.clone());

    let mut default_category_storage =
        toml_table_type.lookup(format!("{}.store", BODY_ENTRY_NAME).as_str())
            .unwrap()
            .as_str()
            .unwrap();
    if matches.is_present(commands::OVERRIDE_SUBCMD) {
        println!("[DEBUG] Got {} command !", commands::OVERRIDE_SUBCMD);
        default_category_storage = matches.subcommand_matches(commands::OVERRIDE_SUBCMD)
            .unwrap()
            .value_of(commands::OVERRIDE_SUBCMD_CATEGORY_FLAG)
            .unwrap();
    }

    // Declare && initialize repositories vectors
    let mut vec_watched = Vec::new();
    let mut vec_path_watched = Vec::new();
    let mut vec_ignored = Vec::new();
    let mut vec_path_ignored = Vec::new();
    // Store watched and ignored git path repositories, from the configuration file
    for (key, value) in toml_table.iter() {
        let category_separator_index = key.find('.');
        if category_separator_index.is_none() {
            continue;
        }
        let key_path: String =
            value.as_table().unwrap().get("path").unwrap().as_str().unwrap().to_owned();
        unsafe {
            let based_key = key.slice_unchecked(category_separator_index.unwrap() + 1, key.len())
                .to_owned();
            if key.starts_with(WATCHED_ENTRY_NAME) && (key != WATCHED_ENTRY_NAME) {
                vec_watched.push(based_key);
                vec_path_watched.push(key_path);
            } else {
                if key.starts_with(IGNORED_ENTRY_NAME) && (key != IGNORED_ENTRY_NAME) {
                    vec_ignored.push(based_key);
                    vec_path_ignored.push(key_path);
                }
            }
        }
    }

    // Get statuses
    if matches.is_present(commands::STATUS_SUBCMD) {
        println!("[DEBUG] Got {} command !", commands::STATUS_SUBCMD);
        git::get_statuses_from(&vec_path_watched);
    }

    if matches.is_present(commands::SCAN_SUBCMD) {
        println!("[DEBUG] Got {} command !", commands::SCAN_SUBCMD);
        // Get local git path directories
        let mut gitpath_directories: Vec<String> = Vec::new();
        find_git_repositories(&mut gitpath_directories, &env::home_dir().unwrap());
        // Get git repositories that are not in an hidden path
        let filtered_git_repositories = filter_hidden_repositories(&gitpath_directories);

        // Compiler error when using pattern matching - TODO
        let entry_category = if default_category_storage == WATCHED_ENTRY_NAME {
            EntryCategory::Watched
        } else {
            EntryCategory::Ignored
        };

        // Filter local git repository, and add them
        for gitrepo in &filtered_git_repositories {
            let gitrepo_name = gitrepo.split("/").last().unwrap();
            let gitrepo_name_s = String::from(gitrepo_name);
            if !(vec_watched.contains(&gitrepo_name_s) || vec_ignored.contains(&gitrepo_name_s)) {
                if !(matches.subcommand_matches(commands::SCAN_SUBCMD)
                    .unwrap()
                    .is_present(commands::SCAN_SUBCMD_DIFF_FLAG)) {
                    match toml_table.add_entry(gitrepo_name,
                                               &mut Entry::new(gitrepo_name, gitrepo),
                                               &entry_category) {
                        Ok(_) => {
                            match entry_category {
                                EntryCategory::Watched => vec_watched.push(gitrepo_name_s),
                                EntryCategory::Ignored => vec_ignored.push(gitrepo_name_s),
                                EntryCategory::Groups => (),
                            }
                            println!("{} has been added to {:?}",
                                     gitrepo_name,
                                     configuration_file_path);
                        }
                        Err(error) => println!("[ERROR] {:?}", error),
                    }
                } else {
                    println!("[DEBUG] Got {} flag !", commands::SCAN_SUBCMD_DIFF_FLAG);
                    println!("Found new repository: {} (in {})", gitrepo_name_s, gitrepo);
                }
            }
        }

        // Save part
        if matches.subcommand_matches(commands::SCAN_SUBCMD)
            .unwrap()
            .is_present(commands::SCAN_SUBCMD_SAVE_FLAG) {
            println!("[DEBUG] Got {} flag !", commands::SCAN_SUBCMD_SAVE_FLAG);
            let mut encoding_str = ConfigurationFile::init();
            match toml_table.encode(&mut encoding_str) {
                Ok(_) => {
                    match encoding_str.save(Path::new(configuration_file_path.as_path())) { 
                        Ok(_) => {
                            println!("The configuration file has been saved in {:?}!",
                                     configuration_file_path)
                        }
                        Err(error) => println!("{}", error),
                    }
                }
                Err(error) => println!("{:?}", error),
            }
        }
    }

}
