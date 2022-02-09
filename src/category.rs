use serde::Deserialize;

use crate::{cmd, TResult};

#[derive(Clone, Debug, Deserialize)]
pub struct Category {
    pub dirs: Vec<String>,

    #[serde(default)]
    pub filetypes: Vec<String>,

    #[serde(default)]
    pub ignored: Vec<String>,

    #[serde(alias = "open-with")]
    pub command: Option<String>,

    #[serde(default)]
    pub wait: bool,
}

impl Category {
    /// Returns a Vec with all files that match against this category.
    pub fn matches(&self) -> TResult<Vec<String>> {
        let result = cmd::exec(&self.query())?;
        Ok(result.lines().map(|s| s.to_string()).collect())
    }

    /// Returns the `find` command that should be run to list all files that
    /// match against this category.
    pub fn query(&self) -> String {
        format!("find {} -type f", self.query_findargs())
    }

    /// Returns the full string of arguments that should be passed to the `find`
    /// command to match against this category.
    fn query_findargs(&self) -> String {
        let dirs = self.query_dirs();
        let include = self.query_include();
        let ignored = self.query_ignored();
        format!("{dirs} {include} {ignored}")
    }

    fn query_dirs(&self) -> String {
        self.dirs.join(" ")
    }

    fn query_ignored(&self) -> String {
        self.ignored.iter().map(|s| format!(" ! -path '*{}'", s)).collect()
    }

    fn query_include(&self) -> String {
        if self.filetypes.is_empty() {
            return String::from("-name '*'");
        };

        let add_flags = |s: &str| format!("-name '*{}' -o ", s);

        let mut out = String::from(r"\( ");
        out.extend(self.filetypes.iter().map(|s| add_flags(s)));
        out.push_str(r"-name '' \)");

        out
    }
}
