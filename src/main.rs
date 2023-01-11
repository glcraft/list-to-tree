use clap::Parser;

#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// File with the list to load
   #[arg(short, long)]
   file: String,
}

#[derive(Debug)]
enum Node {
    Leaf(String),
    Branch(String, Vec<Node>),
}
enum PartitionSize {
    None,
    Many(usize),
    All,
}
impl Node {
    fn partition_size<Str: AsRef<str>, It: Iterator<Item=Str>+Clone>(mut it: It, offset: usize) -> Option<usize> {
        let total = it.clone().count();
        let first_char = match it.next().unwrap().as_ref().chars().nth(offset) {
            Some(c) => c,
            None => return Some(total),
        };
        it.enumerate()
            .find(|(_, word)|{
                match word.as_ref().chars().nth(offset) {
                    Some(c) => c != first_char,
                    None => true,
                }
            })
            .map(|(i, _)| i+1)
            .or(None)
    }
    fn make<Str>(list: &[Str], offset: usize) -> (usize, Option<Self>) 
        where Str: AsRef<str>
    {
        let mut nchar = offset;
        if list.len() == 1 {
            let word = list[0].as_ref()
                .chars()
                .skip(offset)
                .collect::<String>();
            return (1, Some(Node::Leaf(word)));
        }
        let size = Self::partition_size(list.iter(), offset).unwrap();
        assert!(size > 0, "Empty list");
        if size == 1 {
            let word = list[0].as_ref().to_string();
            return (1, Some(Node::Leaf(word)));
        }
        nchar+=1;
        loop {
            match Self::partition_size(list.iter(), nchar) {
                Some(n) if n == size => {},
                _ => break,
            }
            nchar+=1;
        }
        match size {
            0..=1 => {
                unreachable!("Empty list")
            }
            size => {
                let word = list.iter().next()
                    .unwrap()
                    .as_ref()
                    .chars()
                    .skip(offset)
                    .take(nchar - offset)
                    .collect::<String>();
                let mut children = Vec::new();
                let mut i = 0;
                while i < size {
                    let (n, child) = Self::make(&list[i..size], nchar);
                    i += n;
                    if let Some(child) = child {
                        children.push(child);
                    }
                }
                (size, Some(Node::Branch(word, children)))
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    let list_str = std::fs::read_to_string(&args.file).expect("Unable to read file");
    let mut list = list_str.lines().collect::<Vec<_>>();
    list.sort();
    let mut root = Node::make(list.as_slice(), 0).1.unwrap();
    println!("{:#?}", root);
}
