use clap::Parser;

#[derive(Parser)]
#[command(
    name = "commit-helper",
    author = "Mueez Khan",
    about = "Run multiple commands related to `git commit` in succession"
)]
pub struct Cli {
    /// Copy commit message to clipboard instead of committing
    #[arg(short = 'c', long = "clipboard")]
    pub clipboard: bool,

    /// Run `git push` after committing
    #[arg(short = 'p', long = "push")]
    pub push: bool,

    /// Add the `-S` flag when running `git commit` for signing
    #[arg(short = 's', long = "sign")]
    pub sign: bool,

    /// Run in debug mode (print output of each command)
    #[arg(short = 'd', long = "debug")]
    pub debug: bool,

    /// Run in dry run mode (disallow executing commands)
    #[arg(long = "dry-run")]
    pub dry_run: bool,

    /// Run `git commit` with the given message
    /// (if not provided, will prompt for message)
    #[arg(short = 'm', long = "message")]
    pub message: Option<String>,
}

pub struct CommitType {
    pub name: String,
    pub description: String,
}

impl CommitType {
    pub fn new(name: &str, description: &str) -> Self {
        Self {
            name: name.to_owned(),
            description: description.to_owned(),
        }
    }
}

impl std::fmt::Display for CommitType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.description)
    }
}
