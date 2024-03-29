use serde::Deserialize;

use crate::cmd;

#[derive(Clone, Debug, Deserialize)]
pub struct Category {
    pub dirs: Vec<String>,

    #[serde(default)]
    pub filetypes: Vec<String>,

    #[serde(default)]
    pub include: Vec<String>,

    #[serde(default)]
    pub ignored: Vec<String>,

    #[serde(alias = "open-with")]
    pub command: Option<String>,

    #[serde(default)]
    #[serde(alias = "full-path")]
    pub use_full_path: bool,

    #[serde(default)]
    #[serde(alias = "wait")]
    pub terminal: bool,
}

impl Category {
    /// Returns a Vec with all files that match against this category.
    pub fn matches(&self) -> Result<Vec<String>, std::io::Error> {
        let result = cmd::exec(&self.query())?;
        Ok(result.lines().map(|s| s.to_string()).collect())
    }

    /// Returns the `find` command that should be run to list all files that
    /// match against this category.
    pub fn query(&self) -> String {
        format!("find -L {} -type f", self.query_findargs())
    }

    /// Returns the full string of arguments that should be passed to the `find`
    /// command to match against this category.
    fn query_findargs(&self) -> String {
        vec![
            self.query_dirs(),
            self.query_include(),
            self.query_filetypes(),
            self.query_ignored(),
        ]
        .join(" ")
    }

    fn query_dirs(&self) -> String {
        self.dirs.join(" ")
    }

    fn query_ignored(&self) -> String {
        let with_flags = |s: &str| format!("! -path '*{}'", s);
        make_query(&self.ignored, "", with_flags)
    }

    fn query_include(&self) -> String {
        let with_flags = |s: &str| format!("-path '{}'", s);
        make_query(&self.include, "-o", with_flags)
    }

    fn query_filetypes(&self) -> String {
        if self.filetypes.is_empty() {
            return String::from("-name '*'");
        };

        let with_flags = |s: &str| format!("-name '*{}'", s);
        make_query(&self.filetypes, "-o", with_flags)
    }
}

/// Takes a slice of strings, formats them using a given closure, joins them
/// with a given separator, and returns the result.
///
/// ```rust
/// let items = ["foo".to_string(), "bar".to_string(), "baz".to_string()];
/// let with_flags = |s: &str| format!("-name '*{}'", s);
///
/// let query = make_query(items, "-o", with_flags);
/// assert_eq(&query, r"\( -name foo -o -name bar -o -name baz \)");
/// ```
fn make_query(
    options: &[String], separator: &str, map: impl Fn(&str) -> String,
) -> String {
    if options.is_empty() {
        return String::new();
    }

    let separator = format!(" {} ", separator);
    let flags =
        options.iter().map(|s| map(s)).collect::<Vec<_>>().join(&separator);

    format!("\\( {} \\)", flags)
}
