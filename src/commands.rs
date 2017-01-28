use clap::{App, Arg, ArgMatches, SubCommand};
use libgpm::{IGNORED_ENTRY_NAME, WATCHED_ENTRY_NAME};

// Program relative

/// Name of the program
static PRG_NAME: &'static str = "gpm";

// Flags

/// Reset flag
pub static RESET_FLAG: &'static str = "reset";
static RESET_FLAG_SHORT: &'static str = "r";

// Subcommands

/// Diff subcommand
pub static DIFF_SUBCMD: &'static str = "diff";

/// Store subcommand
pub static STORE_SUBCMD: &'static str = "store";

pub static STORE_SUBCMD_DEFAULT_FLAG: &'static str = "default";
static STORE_SUBCMD_DEFAULT_FLAG_SHORT: &'static str = "d";

/// Function to get arguments of the program.
/// This function returns an ArgMatches type.
pub fn get_program_args<'a>() -> ArgMatches<'a> {
    App::new(PRG_NAME)
        .version(crate_version!())
        .author("A. Carette <antonin@carette.xyz>")
        .about("Your Git Project Manager")
        .arg(Arg::with_name(RESET_FLAG)
            .short(RESET_FLAG_SHORT)
            .long(RESET_FLAG)
            .help("Reset the entire configuration file to the default values"))
        .subcommand(SubCommand::with_name(DIFF_SUBCMD)
            .author("A. Carette <antonin@carette.xyz>")
            .about("Get the difference between current state and new local git repositories \
                    unfollowed"))
        .subcommand(SubCommand::with_name(STORE_SUBCMD)
            .author("A. Carette <antonin@carette.xyz>")
            .about("Manage storing behaviours, for new git repositories")
            .arg(Arg::with_name(STORE_SUBCMD_DEFAULT_FLAG)
                .short(STORE_SUBCMD_DEFAULT_FLAG_SHORT)
                .long(STORE_SUBCMD_DEFAULT_FLAG)
                .help("Default 'location' of new git repositories")
                .takes_value(true)
                .possible_values(&[IGNORED_ENTRY_NAME, WATCHED_ENTRY_NAME])
                .default_value(WATCHED_ENTRY_NAME)))
        .get_matches()
}
