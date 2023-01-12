use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Formatter {
    /// Output the tree into a regex
    Regex,
    /// Output the tree into rust ast
    Rust,
}

/// Argument model for the command line interface.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the file containing the list of words.
    /// Defaults to stdin
    #[arg(short, long)]
    pub input: Option<String>,
    /// Path to the file where the output will be written.
    /// Defaults to stdout
    #[arg(short, long)]
    pub output: Option<String>,
    /// Format of the output.
    #[arg(long)]
    pub format: Formatter,
    /// Pretty print the output.
    #[arg(long)]
    pub pretty: bool,
}