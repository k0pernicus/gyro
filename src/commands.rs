use clap::{App, Arg, ArgMatches, SubCommand};
use libgpm::{IGNORED_ENTRY_NAME, WATCHED_ENTRY_NAME};

// Program relative

/// Name of the program
pub static PRG_NAME: &'static str = "gpm";

// Flags

/// Reset flag
pub static RESET_FLAG: &'static str = "reset";
static RESET_FLAG_SHORT: &'static str = "r";

// Subcommands

///  Override subcommand
pub static OVERRIDE_SUBCMD: &'static str = "override";
pub static OVERRIDE_SUBCMD_CATEGORY_FLAG: &'static str = "category";

/// Scan subcommand
pub static SCAN_SUBCMD: &'static str = "scan";
pub static SCAN_SUBCMD_DIFF_FLAG: &'static str = "diff";
pub static SCAN_SUBCMD_SAVE_FLAG: &'static str = "save";

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
        .subcommand(SubCommand::with_name(OVERRIDE_SUBCMD)
            .author("A. Carette <antonin@carette.xyz>")
            .about("Override default settings from your configuration file")
            .arg(Arg::with_name(OVERRIDE_SUBCMD_CATEGORY_FLAG)
                .help("Override the default location of new git repositories")
                .takes_value(true)
                .possible_values(&[IGNORED_ENTRY_NAME, WATCHED_ENTRY_NAME])
                .long(OVERRIDE_SUBCMD_CATEGORY_FLAG)))
        .subcommand(SubCommand::with_name(SCAN_SUBCMD)
            .author("A. Carette <antonin@carette.xyz>")
            .about("Scan your hard disk to find git repositories")
            .arg(Arg::with_name(SCAN_SUBCMD_DIFF_FLAG)
                .help("Print new git repositories from your hard disk")
                .conflicts_with(SCAN_SUBCMD_SAVE_FLAG)
                .long(SCAN_SUBCMD_DIFF_FLAG))
            .arg(Arg::with_name(SCAN_SUBCMD_SAVE_FLAG)
                .help("Save new git repositories into your configuration file")
                .long(SCAN_SUBCMD_SAVE_FLAG)))
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
