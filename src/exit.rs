use std::error::Error;

use crate::cli;

pub fn exit_with_help(program: &str, opts: getopts::Options) -> ! {
    println!("{}", cli::usage(program, opts));
    std::process::exit(0);
}

pub fn exit_with_error(error: Box<dyn Error>) -> ! {
    eprintln!("{}", error);
    std::process::exit(1);
}
