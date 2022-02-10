mod category;
mod cli;
mod cmd;
mod config;
mod exit;

type TResult<T> = Result<T, Box<dyn std::error::Error>>;

fn main() {
    run().unwrap_or_else(|e| exit::exit_with_error(e))
}

fn run() -> TResult<()> {
    let opts = cli::opts();
    let args = cli::args();
    let matches = opts.parse(args.args())?;

    let mut config = config::read_config()?;

    if matches.opt_present("h") {
        exit::exit_with_help(args.program(), opts);
    }

    let chosen_category: &str = match matches.free.is_empty() {
        true => exit::exit_with_help(args.program(), opts),
        false => &matches.free[0],
    };

    let category =
        config.categories.get_mut(chosen_category).unwrap_or_else(|| {
            exit::exit_with_error(
                format!("unknown category: {}", chosen_category).into(),
            )
        });

    if category.filetypes.is_empty() {
        category.filetypes.push(String::from("*"));
    }

    let mut files = category.matches()?;
    if files.is_empty() {
        exit::exit_with_error("no files found".into());
    }
    else {
        files.sort();
    }

    let file = cmd::user_select(&files)?;
    let file = file.split(' ').collect::<Vec<_>>().join(r"\ ");

    if !file.is_empty() {
        let mut child = cmd::spawn_opener(category, &file);

        if category.terminal {
            child.wait()?;
        }
    }

    Ok(())
}
