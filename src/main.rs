fn main() {
    let opts = cli::opts();
    let (program, args) = cli::args();

    let matches = opts
        .parse(&args)
        .unwrap_or_else(|e| exit::exit_with_error(&e.to_string(), 1));

    if matches.opt_present("h") {
        exit::exit_with_help(&program, opts);
    }

    let mode: &str = match matches.free.is_empty() {
        true => exit::exit_with_help(&program, opts),
        false => &matches.free[0],
    };

    println!("{mode}");
}

mod cli {
    use std::env;

    pub fn opts() -> getopts::Options {
        let mut opts = getopts::Options::new();

        opts.optflag("h", "help", "print this help menu");

        opts
    }

    pub fn args() -> (String, Vec<String>) {
        let mut args = env::args();
        let program = args.next().unwrap();
        (program, args.collect())
    }

    pub fn usage(program: &str, opts: getopts::Options) -> String {
        let brief = format!("Usage: {} FILE [options]", program);
        format!("{}", opts.usage(&brief))
    }
}

mod exit {
    use crate::cli;

    pub fn exit_with_help(program: &str, opts: getopts::Options) -> ! {
        println!("{}", cli::usage(&program, opts));
        std::process::exit(0);
    }

    pub fn exit_with_error(error: &str, code: i32) -> ! {
        eprintln!("{}", error);
        std::process::exit(code);
    }
}
