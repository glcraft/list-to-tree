use clap::Parser;
mod node;
#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// File with the list to load
   #[arg(short, long)]
   file: String,
}

fn main() {
    let args = Args::parse();
    let list_str = std::fs::read_to_string(&args.file).expect("Unable to read file");
    let mut list = list_str.lines().collect::<Vec<_>>();
    list.sort();
    let root = node::Node::new(list.as_slice());
    // println!("{:#?}", root);
}
