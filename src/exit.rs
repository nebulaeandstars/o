//! Functions for controlled (pretty) exiting.

use std::error::Error;

use crate::cli;

/// Extract usage information from a set of options, print it, and exit.
pub fn exit_with_help(program: &str, opts: getopts::Options) -> ! {
    println!("{}", cli::usage(program, opts));
    std::process::exit(0);
}

/// Print a given error to stderr and exit with error code 1.
pub fn exit_with_error(error: Box<dyn Error>) -> ! {
    eprintln!("{}", error);
    std::process::exit(1);
}
