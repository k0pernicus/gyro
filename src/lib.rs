extern crate ansi_term;
extern crate chrono;
extern crate git2;
extern crate rustc_serialize;
extern crate toml;
extern crate walkdir;

pub mod configuration;
pub mod file;
pub mod git;
pub mod scan;

use toml::{Encoder, Table};

///
/// Static variable to get the name of the git main directory
///
static GIT_DIR_NAME: &'static str = ".git";

///
/// Static variable to get the name of the global configuration file
///
static CONFIGURATION_FILE_NAME: &'static str = ".gpm";

///
/// Static variable to get the name of the global configuration file copy
///
static CONFIGURATION_FILE_NAME_BUP: &'static str = ".gpm.new";

///
/// Static variable to get the entry name of watched git repositories
///
static WATCHED_ENTRY_NAME: &'static str = "watched";

///
/// Static variable to get the entry name of ignored git repositories
///
static IGNORED_ENTRY_NAME: &'static str = "ignored";

///
/// Static variable to get the entry name of git repo groups
///
static GROUPS_ENTRY_NAME: &'static str = "groups";

/// The type of the content file is a Table type.
pub type ConfigurationContent = Table;

/// The configuration file is basically a TOML file that contain some informations about local git
/// projects.
pub type ConfigurationFile = Encoder;
