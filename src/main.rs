mod node;
mod regex;
mod arguments;

use std::io::{Read, Write};
use clap::Parser;
use arguments::Args;

fn main() {
    let args = Args::parse();
    let list_str = match args.input {
        Some(path) => std::fs::read_to_string(path).expect("Unable to read file"),
        None => {
            let mut list_str = String::new();
            std::io::stdin().read_to_string(&mut list_str).expect("Unable to read stdin");
            list_str
        }
    };
    let mut list = list_str.lines().collect::<Vec<_>>();
    list.sort();
    list.dedup();
    let root = node::Node::new(list.as_slice());
    let mut file : Box<dyn Write> = match args.output {
        Some(path) => Box::new(std::fs::File::create(path).expect("Unable to create file")),
        None => Box::new(std::io::stdout())
    };
    let output = match args.format {
        arguments::Formatter::Regex => regex::Regex::from(root).0,
        arguments::Formatter::Rust => if args.pretty {
            format!("{:#?}", root)
        } else {
            format!("{:?}", root)
        }
    };
    file.write_all(output.as_bytes()).expect("Unable to write to file");
}
