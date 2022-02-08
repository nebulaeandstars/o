fn main() {
    let opts = cli::opts();
    let args = cli::args();

    let matches =
        opts.parse(args.args()).unwrap_or_else(|e| exit::exit_with_error(e));

    if matches.opt_present("h") {
        exit::exit_with_help(args.program(), opts);
    }

    let mode: &str = match matches.free.is_empty() {
        true => exit::exit_with_help(args.program(), opts),
        false => &matches.free[0],
    };

    println!("{}", mode);
}

mod cli {
    use std::env;

    pub fn opts() -> getopts::Options {
        let mut opts = getopts::Options::new();

        opts.optflag("h", "help", "print this help menu");

        opts
    }

    pub struct Args {
        program: String,
        args:    Vec<String>,
    }

    impl Args {
        pub fn program(&self) -> &str {
            &self.program
        }
        pub fn args(&self) -> &[String] {
            &self.args
        }
    }

    pub fn args() -> Args {
        let mut args = env::args();
        let program: String = args.next().unwrap();
        let args: Vec<String> = args.collect();
        Args { program, args }
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

    pub fn exit_with_error(error: impl std::error::Error) -> ! {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}
