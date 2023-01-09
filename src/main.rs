use std::collections::HashMap;

use clap::Parser;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// File with the list to load
   #[arg(short, long)]
   file: String,
}

#[derive(Debug)]
enum Node<'a> {
    Leaf(Vec<&'a str>),
    Branch(HashMap<&'a str, Node<'a>>),
}
impl<'a> Node<'a> {
    fn find_difference(a: &'a str, b: &'a str) -> usize {
        assert_ne!(a, b, "List elements not distinct");
        a.chars()
            .zip(b.chars())
            .enumerate()
            .find(|(_, (a, b))| a != b)
            .map(|(i, _)| i)
            .or(Some(a.len()))
        
    }
    fn is_similar_from(a: &'a str, b: &'a str, diff: usize) -> bool {
        a.chars()
            .zip(b.chars())
            .skip(diff)
            .all(|(a, b)| a == b)
    }
    fn make<It: Iterator<Item=(&'a str, &'a str)>>(&mut self, mut it: It) {
        if let Some((a, b)) = it.next() {
            let diff = Self::find_difference(a, b);
            let (a, b) = a.split_at(diff);
            let mut map = HashMap::new();
            map.insert(b, Node::Leaf(vec![a]));
            self.make(it);
            *self = Node::Branch(map);
        }
    }
}

fn main() {
    let args = Args::parse();
    let list_str = std::fs::read_to_string(&args.file).expect("Unable to read file");
    let mut list = list_str.lines().collect::<Vec<_>>();
    list.sort();
    let mut root = Node::new("");
    println!("{:#?}", root);
}
