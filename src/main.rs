mod cli;
mod cmd;
mod config;
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

    let config = config::read_config()?;

    if matches.opt_present("h") {
        exit::exit_with_help(args.program(), opts);
    }

    let chosen_category: &str = match matches.free.is_empty() {
        true => exit::exit_with_help(args.program(), opts),
        false => &matches.free[0],
    };

    let category =
        config.categories.get(chosen_category).unwrap_or_else(|| {
            exit::exit_with_error(
                format!("unknown category: {}", chosen_category).into(),
            )
        });

    let mut files = cmd::find_files(
        &category.dirs,
        &category.ignored,
        &category.filetypes,
    )?;
    if files.is_empty() {
        exit::exit_with_error("no files found".into());
    }
    else {
        files.sort();
    }

    let file = cmd::user_select(&files)?;
    let file = file.split(" ").collect::<Vec<_>>().join(r"\ ");
    let path = format!("{}", file);

    let command = match &category.command {
        Some(command) => format!("{} {}", command, path),
        None => format!("xdg-open {}", path),
    };

    let crash = || {
        exit::exit_with_error(
            format!("error executing command: {}", &command).into(),
        )
    };

    if !file.is_empty() {
        println!("{}", command);

        let mut child = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .spawn()
            .unwrap_or_else(|_| crash());

        if category.wait {
            child.wait()?;
        }
    }

    Ok(())
}
