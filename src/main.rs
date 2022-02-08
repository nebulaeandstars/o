mod cli;
mod cmd;
mod exit;

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

    let dirs = &["~/docs/calibre/", "~/docs/guides/rust"];
    let filetypes = &[".pdf", ".jpg"];
    let files = cmd::find_files(dirs, filetypes)?;

    let file = cmd::user_select(&files)?;
    let path = format!("{}", file);

    if !file.trim().is_empty() {
        Command::new("xdg-open").arg(path).spawn()?;
    }

    Ok(())
}
