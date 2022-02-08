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

    let files = cmd::find_files(&category.dirs, &category.filetypes)?;

    let file = cmd::user_select(&files)?;
    let path = format!("{}", file);

    if !file.trim().is_empty() {
        if let Some(command) = &category.command {
            Command::new(command).arg(path).spawn()?.wait().unwrap_or_else(
                |_| {
                    exit::exit_with_error(
                        format!("error executing command: {}", command).into(),
                    )
                },
            );
        }
        else {
            Command::new("xdg-open").arg(path).spawn()?;
        }
    }

    Ok(())
}
