use smartstring::alias::String;


#[derive(Debug)]
pub enum Node {
    Leaf(String),
    Branch(String, Vec<Node>),
}
enum PartitionSize {
    None,
    Many(usize),
    All,
}
impl Node {
    fn partition_size<Str: AsRef<str>, It: Iterator<Item=Str>+Clone>(mut it: It, offset: usize) -> PartitionSize {
        let first_char = match it.next().expect("Empty list").as_ref().chars().nth(offset) {
            Some(c) => c,
            None => return PartitionSize::All,
        };
        let result = it.enumerate()
            .find(|(_, word)|{
                match word.as_ref().chars().nth(offset) {
                    Some(c) => c != first_char,
                    None => true,
                }
            })
            .map(|(i, _)| i+1);
        match result {
            Some(n) => PartitionSize::Many(n),
            None => PartitionSize::None,
        }
    }
    fn make_leaf<Str>(list: &[Str], offset: usize) -> Node 
        where Str: AsRef<str>
    {
        let word = list[0]
            .as_ref()
            .chars()
            .skip(offset)
            .collect::<String>();
        Node::Leaf(word)
    }
    fn make_branch<Str>(list: &[Str], offset: usize) -> Node 
        where Str: AsRef<str>
    {
        let mut nchar = offset;
        loop {
            match Self::partition_size(list.iter(), nchar) {
                PartitionSize::None => nchar+=1,
                _ => break,
            }
        }
        let word = list[0]
            .as_ref()
            .chars()
            .skip(offset)
            .take(nchar - offset)
            .collect::<String>();
        let (mut i, mut children) = if word == "" {
            (1, vec![Node::Leaf(String::new())])
        } else {
            (0, Vec::new())
        };
        while i < list.len() {
            let (n, child) = Self::make(&list[i..list.len()], nchar);
            i += n;
            children.push(child);
        }
        Node::Branch(word, children)
    }
    fn make<Str>(list: &[Str], offset: usize) -> (usize, Self) 
        where Str: AsRef<str>
    {
        let size = match Self::partition_size(list.iter(), offset) {
            PartitionSize::None => list.len(),
            PartitionSize::Many(n) => n,
            PartitionSize::All => list.len()
        };
        match size {
            0 => unreachable!("Empty list"),
            1 => (1, Self::make_leaf(&list[0..1], offset)),
            size => (size, Self::make_branch(&list[0..size], offset))
        }
    }
    pub fn new<Str>(list: &[Str]) -> Self
        where Str: AsRef<str>
    {
        match list.len() {
            0 => Self::Leaf(String::new()),
            1 => Self::make_leaf(list, 0),
            _ => Self::make_branch(list, 0)
        }
    }
}
