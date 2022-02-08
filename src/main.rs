use std::process::Command;

type TResult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() {
    run().unwrap_or_else(|e| exit::exit_with_error(e))
}

fn run() -> TResult<()> {
    let opts = cli::opts();
    let args = cli::args();

    let matches = opts.parse(args.args())?;

    if matches.opt_present("h") {
        exit::exit_with_help(args.program(), opts);
    }

    let mode: &str = match matches.free.is_empty() {
        true => exit::exit_with_help(args.program(), opts),
        false => &matches.free[0],
    };

    let dir = mode;
    let _ = cmd::navigate(dir)?;

    let files = cmd::list_files()?;
    let file = cmd::user_select(files)?;
    let path = format!("{}", file);

    if !file.trim().is_empty() {
        Command::new("xdg-open").arg(path).spawn()?;
    }

    Ok(())
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
        opts.usage(&brief)
    }
}

mod cmd {
    use std::env;
    use std::io::Write;
    use std::process::{Command, Stdio};

    use crate::TResult;

    pub fn navigate(dir: &str) -> TResult<std::path::PathBuf> {
        env::set_current_dir(dir).map_err(|e| format!("{} - {}", dir, e))?;
        env::current_dir().map_err(|e| e.into())
    }

    pub fn user_select(list: String) -> TResult<String> {
        let mut finder = finder()?;
        let mut stdin = finder.stdin.take().expect("Failed to open stdin");
        std::thread::spawn(move || {
            stdin.write_all(list.as_bytes()).expect("Failed to write to stdin");
        });

        Ok(finder
            .wait_with_output()?
            .stdout
            .into_iter()
            .map(|c| c as char)
            .collect())
    }

    pub fn list_files() -> TResult<String> {
        let mut out = String::new();
        let files = exec(r"ls -1")?;

        for file in files.lines() {
            out.push_str(&exec(&format!("find {:?} -type f", file))?);
        }

        Ok(out)
    }

    pub fn exec(command: &str) -> TResult<String> {
        Ok(Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()?
            .stdout
            .into_iter()
            .map(|c| c as char)
            .collect::<String>())
    }

    pub fn finder() -> TResult<std::process::Child> {
        // If in a tty, try to use fzf.
        if atty::is(atty::Stream::Stdout) {
            let result = Command::new("fzf")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn();

            if let Ok(child) = result {
                return Ok(child);
            }
        }

        // If not in a tty, or if fzf didn't work, try to use dmenu.
        Ok(Command::new("dmenu")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?)
    }
}

mod exit {
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
}
