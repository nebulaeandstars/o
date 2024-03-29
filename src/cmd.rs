//! External (shell) process calls.

use std::io::Write;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

use crate::category::Category;
use crate::exit;

pub fn user_select(
    list: &[String], use_full_path: bool,
) -> Result<&'_ str, Box<dyn std::error::Error>> {
    let mut finder = finder();
    let mut stdin = finder.stdin.take().expect("Failed to open stdin");

    let fmt_path = |path: &String| {
        if use_full_path {
            PathBuf::from(path).to_str().unwrap().to_owned()
        }
        else {
            PathBuf::from(path)
                .file_name()
                .unwrap()
                .to_owned()
                .into_string()
                .unwrap()
        }
    };

    let query = list
        .iter()
        .map(fmt_path)
        .enumerate()
        .rev()
        .map(|(i, filename)| format!("{} ({})", filename, i))
        .collect::<Vec<_>>()
        .join("\n");

    std::thread::spawn(move || {
        stdin.write_all(query.as_bytes()).expect("Failed to write to stdin");
    });

    let selection: String = finder
        .wait_with_output()?
        .stdout
        .into_iter()
        .map(|c| c as char)
        .collect();

    if !selection.is_empty() {
        let index = selection
            .split(' ')
            .next_back()
            .map(|s| &s[1..(s.len() - 2)])
            .unwrap()
            .parse::<usize>()?;
        Ok(&list[index])
    }
    else {
        Err("".into())
    }
}

pub fn spawn_opener(category: &Category, filepath: &str) -> Child {
    let command = match &category.command {
        Some(command) => format!("{} {}", command, filepath),
        None => format!("xdg-open {}", filepath),
    };

    let crash = || {
        exit::exit_with_error(
            format!("error executing command: {}", &command).into(),
        );
    };

    if atty::is(atty::Stream::Stdout) {
        println!("{}", command);
    }

    if category.terminal
        && !atty::is(atty::Stream::Stdin)
        && !atty::is(atty::Stream::Stdout)
    {
        Command::new("st")
            .arg("-e")
            .arg("sh")
            .arg("-c")
            .arg(&command)
            .spawn()
            .unwrap_or_else(|_| crash())
    }
    else {
        Command::new("sh")
            .arg("-c")
            .arg(&command)
            .spawn()
            .unwrap_or_else(|_| crash())
    }
}

pub fn exec(command: &str) -> Result<String, std::io::Error> {
    Ok(Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()?
        .stdout
        .into_iter()
        .map(|c| c as char)
        .collect::<String>())
}

pub fn finder() -> std::process::Child {
    // If in a tty, try to use fzf.
    if atty::is(atty::Stream::Stdin) && atty::is(atty::Stream::Stdout) {
        let result = Command::new("fzf")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn();

        if let Ok(child) = result {
            return child;
        }
    }

    let crash = |_| {
        exit::exit_with_error(
            "search error: could not spawn fzf or dmenu".into(),
        );
    };

    // If not in a tty, or if fzf didn't work, try to use dmenu.
    Command::new("dmenu")
        .arg("-l")
        .arg("20")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap_or_else(crash)
}
