use clap::{App, Arg, ArgMatches, SubCommand};
use libgyro::{IGNORED_ENTRY_NAME, WATCHED_ENTRY_NAME};

// Program relative

/// Name of the program
pub static PRG_NAME: &'static str = "gyro";

// Flags

/// Reset flag
pub static RESET_FLAG: &'static str = "reset";
static RESET_FLAG_SHORT: &'static str = "r";

/// Save flag
pub static SAVE_FLAG: &'static str = "save";
static SAVE_FLAG_SHORT: &'static str = "s";

// Subcommands

///  Override subcommand
pub static OVERRIDE_SUBCMD: &'static str = "override";
pub static OVERRIDE_SUBCMD_CATEGORY_FLAG: &'static str = "category";

/// Move subcommand
pub static REPO_SUBCMD: &'static str = "repo";
pub static REPO_SUBCMD_MOVE_FLAG: &'static str = "move";
pub static REPO_SUBCMD_NAME_FLAG: &'static str = "name";

/// Scan subcommand
pub static SCAN_SUBCMD: &'static str = "scan";
pub static SCAN_SUBCMD_DIFF_FLAG: &'static str = "diff";

/// Status subcommand
pub static STATUS_SUBCMD: &'static str = "status";
pub static STATUS_SUBCMD_CLEAN_FLAG: &'static str = "clean";
pub static STATUS_SUBCMD_DIRTY_FLAG: &'static str = "dirty";

/// Function to get arguments of the program.
/// This function returns an ArgMatches type.
pub fn get_program_args<'a>() -> ArgMatches<'a> {
    App::new(PRG_NAME)
        .version(crate_version!())
        .author("A. Carette <antonin@carette.xyz>")
        .about("Your Git Project Monitor")
        .arg(Arg::with_name(RESET_FLAG)
            .short(RESET_FLAG_SHORT)
            .long(RESET_FLAG)
            .help("Reset the configuration file"))
        .arg(Arg::with_name(SAVE_FLAG)
            .short(SAVE_FLAG_SHORT)
            .long(SAVE_FLAG)
            .help("Save the current configuration file"))
        .subcommand(SubCommand::with_name(OVERRIDE_SUBCMD)
            .author("A. Carette <antonin@carette.xyz>")
            .about("Override default settings from your configuration file")
            .arg(Arg::with_name(OVERRIDE_SUBCMD_CATEGORY_FLAG)
                .help("Override the default location of new git repositories")
                .takes_value(true)
                .possible_values(&[IGNORED_ENTRY_NAME, WATCHED_ENTRY_NAME])
                .long(OVERRIDE_SUBCMD_CATEGORY_FLAG)))
        .subcommand(SubCommand::with_name(REPO_SUBCMD)
            .author("A. Carette <antonin@carette.xyz>")
            .about("Play with a given local git repository")
            .arg(Arg::with_name(REPO_SUBCMD_NAME_FLAG)
                .help("The local git repository to play with")
                .index(1)
                .required(true))
            .arg(Arg::with_name(REPO_SUBCMD_MOVE_FLAG)
                .help("Move the given repository to the other repositories container (watched, \
                       or ignored)")
                .long(REPO_SUBCMD_MOVE_FLAG)))
        .subcommand(SubCommand::with_name(SCAN_SUBCMD)
            .author("A. Carette <antonin@carette.xyz>")
            .about("Scan your hard disk to find git repositories")
            .arg(Arg::with_name(SCAN_SUBCMD_DIFF_FLAG)
                .help("Print new git repositories from your hard disk")
                .long(SCAN_SUBCMD_DIFF_FLAG)))
        .subcommand(SubCommand::with_name(STATUS_SUBCMD)
            .author("A. Carette <antonin@carette.xyz>")
            .about("Get the status of watched git repositories")
            .arg(Arg::with_name(STATUS_SUBCMD_CLEAN_FLAG)
                .help("Get only clean projects")
                .long(STATUS_SUBCMD_CLEAN_FLAG)
                .conflicts_with(STATUS_SUBCMD_DIRTY_FLAG))
            .arg(Arg::with_name(STATUS_SUBCMD_DIRTY_FLAG)
                .help("Get only dirty projects")
                .long(STATUS_SUBCMD_DIRTY_FLAG)))
        .get_matches()
}
