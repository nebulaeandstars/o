//! External (shell) process calls.

use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use crate::TResult;

pub fn user_select<'a>(list: &'a [String]) -> TResult<&'a str> {
    let mut finder = finder()?;
    let mut stdin = finder.stdin.take().expect("Failed to open stdin");

    let query = list
        .iter()
        .map(|path| PathBuf::from(path).file_name().unwrap().to_owned())
        .enumerate()
        .rev()
        .map(|(i, filename)| format!("{} {:#?}", i, filename))
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
        let index = selection.split(" ").next().unwrap().parse::<usize>()?;
        Ok(&list[index])
    }
    else {
        Err("".into())
    }
}

pub fn find_files(
    dirs: &[String], ignored: &[String], filetypes: &[String],
) -> TResult<Vec<String>> {
    let dirs_query = dirs.join(" ");
    let mut filetypes_query = String::new();

    // add file extensions to the query
    filetypes_query.push_str(r"\( ");
    filetypes_query
        .extend(filetypes.iter().map(|s| format!("-name '*{}' -o ", s)));
    filetypes_query.push_str(r"-name '' \)");

    // add ignored patterns to the query
    filetypes_query
        .extend(ignored.iter().map(|s| format!(" ! -path '*{}'", s)));

    let query = format!("find {} {} -type f", dirs_query, filetypes_query);

    let files = exec(&query)?;
    let out = files.lines().map(|line| line.to_owned()).collect();
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
    if atty::is(atty::Stream::Stdin) && atty::is(atty::Stream::Stdout) {
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
        .arg("-l")
        .arg("20")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?)
}
