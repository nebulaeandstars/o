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
    opts.usage(&brief)
}
